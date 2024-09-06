use crate::{
    core::{
        check::checker::Checker,
        compiler::compile_runner::CompileRunner,
        core_error::CoreInitError,
        editor::opener::EditorOpener,
        file_utils::{build_folder::generate_build_folder, file_utils::current_folder},
        launcher::launcher::Launcher,
        parser::from_dir::FromDir,
        watcher::watcher::FileWatcher,
        work::{work::Work, work_handler::WorkHandler, work_type::WorkType},
    },
    models::{
        check_state::CheckStatus, constants::TARGET_FILE_BASE_NAME, event::Event, exo::Exo,
        project::Project, ui_state::UiState,
    },
    ui::ui::Ui,
};
use log::{error, info};
use std::{
    path::PathBuf,
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
};

use super::{
    errors::{CompilationStartError, StartExoFail},
    exo_status_report::ExoStatusReport,
};

/// App struct
/// Holds the state of the application
pub struct App {
    pub(super) ui_state: UiState,
    pub(super) project: Project,
    pub(super) work_handler: Arc<Mutex<WorkHandler>>,
    pub(super) event_rx: Receiver<Event>,
    pub(super) ui_state_tx: Sender<UiState>,
    pub(super) run: bool,
    pub(super) current_run: Option<ExoStatusReport>,
}

impl App {
    ///  Create a new App instance
    ///
    /// This function will create a new App instance and initialize the project
    /// It will succeed if the project is found in the current folder
    ///
    /// # Returns
    /// A Result containing the App instance if the project is found or an   
    /// error if the project is not found
    ///
    pub fn new() -> Result<Self, CoreInitError> {
        let current_folder = match current_folder() {
            Ok(folder) => folder,
            Err(_err) => return Err(CoreInitError::PlxProjNotFound), // TODO maybe be more specific
                                                                     // here by adding the error detail
        };
        // TODO these warnings should be accessible to the user
        let (project, _warnings) = match Project::from_dir(&current_folder) {
            Ok((project, warnings)) => (project, warnings),
            Err((err, _warnings)) => {
                // TODO handle these warnings even in case of failure
                return Err(CoreInitError::ProjFilesParsingError(format!("{:?}", err)));
            }
        };
        let (event_tx, event_rx) = mpsc::channel();
        let (ui_state_tx, ui_state_rx) = mpsc::channel();
        let work_handler = WorkHandler::new(event_tx.clone());

        let mut app = App {
            ui_state: UiState::Home,
            project,
            work_handler,
            event_rx,
            ui_state_tx,
            run: true,
            current_run: None,
        };
        app.start_ui(ui_state_rx);
        Ok(app)
    }

    /// Sets a new UiState
    /// It's important to set the ui_state using this functions as it will also notify the UI of the change
    pub(super) fn set_ui_state(&mut self, new_state: UiState) {
        //TODO maybe restart the ui if the channel is closed ?
        let _ = self.ui_state_tx.send(new_state.clone());
        self.ui_state = new_state;
    }

    /// Tries to resume the last exo that was being worked on the last time the app was closed  
    ///  
    /// If the last exo is found, it will try to resume it, otherwise it will go to the skill selection screen
    ///
    ///
    pub(super) fn resume_last_exo(&mut self) {
        if let Some(exo) = &self.project.resume() {
            //TODO refactor this code (duplicate)
            match App::start_exo(&self.work_handler, exo) {
                Ok(cr) => {
                    self.current_run = Some(cr);
                    self.go_to_compiling();
                }
                Err(err) => {
                    error!("Couldn't start exo {}", err);
                    self.go_to_skill_selection();
                }
            }
        } else {
            self.go_to_skill_selection();
        }
    }
    /// Main thread
    ///
    /// Main application loop
    pub fn run_forever(mut self) {
        while self.run {
            if let Ok(event) = self.event_rx.recv() {
                info!("{:?}", event);
                match event {
                    Event::KeyPressed(key) => self.on_key_press(key),
                    Event::EditorOpened => {}
                    Event::CouldNotOpenEditor => {} //TODO warn the user ?
                    Event::OutputCheckPassed(check_index) => self.on_check_passed(check_index),
                    Event::OutputCheckFailed(check_index, diff) => {
                        info!("{}", diff.to_ansi_colors());
                        self.on_check_failed(check_index, diff)
                    }
                    Event::FileSaved => self.on_file_save(),
                    Event::CompilationStart => self.on_compilation_start(),
                    Event::CompilationEnd(success) => self.on_compilation_end(success),
                    Event::CompilationOutputLine(line) => self.on_compilation_output(line),
                    Event::RunStart(id) => self.on_run_start(id),
                    Event::RunEnd(id) => self.on_run_end(id),
                    Event::RunOutputLine(id, line) => self.on_run_output(id, line),
                    Event::RunFail(run_id, err) => self.on_run_fail(run_id, err),
                }
            }
        }
    }
    /// Starts a new worker using the work_handler
    ///
    /// Returns the id of the worker if it was successfully started
    /// else it returns None
    ///
    fn start_work(wh: &Arc<Mutex<WorkHandler>>, work: Box<dyn Work + Send>) -> Option<usize> {
        if let Ok(mut wh) = wh.lock() {
            return Some(wh.spawn_worker(work));
        }
        None
    }
    /// Starts the UI
    ///
    /// The UI will be launched as a separate worker so this function will not block
    ///
    fn start_ui(&mut self, ui_state_rx: Receiver<UiState>) {
        let ui = Ui::new(ui_state_rx);
        self.go_to_home();
        App::start_work(&self.work_handler, Box::new(ui));
    }
    /// Stops the UI
    /// Useful if we want to restart the UI
    ///
    fn _stop_ui(&mut self) {
        if let Ok(mut work_handler) = self.work_handler.lock() {
            work_handler.stop_workers(WorkType::Ui);
        }
    }

    /// Opens a new editor using a worker
    /// This function will try to open the main file of the exo using the default system editor
    /// See `EditorOpener` for more details
    ///
    /// This function doesn't block, the editor will be opened using a new worker
    ///
    /// Returns the id of the worker if it was successfully started
    /// else it returns None
    fn open_editor(wh: &Arc<Mutex<WorkHandler>>, exo: &Exo) -> Option<usize> {
        if let Some(file) = exo.get_main_file() {
            if let Some(opener) = EditorOpener::new_default_editor(file.to_path_buf()) {
                return App::start_work(wh, Box::new(opener));
            }
        }
        None
    }
    /// Compiles the exo using a worker
    ///
    /// This function doesn't block, the compilation will be done using a new worker
    ///
    /// Returns the path to the compiled file if the compilation launch was successful
    /// else it returns an error describing why it failed
    pub(super) fn compile(
        wh: &Arc<Mutex<WorkHandler>>,
        exo: &Exo,
    ) -> Result<PathBuf, CompilationStartError> {
        let compiler = exo
            .compiler()
            .ok_or(CompilationStartError::CompilerNotSupported)?;
        info!("Compiler: {:#?}", compiler);

        let folder = generate_build_folder(exo).map_err(|err| {
            error!("Error generation build folder ({})", err);
            CompilationStartError::BuildFolderGenerationFailed
        })?;
        info!("Folder: {:#?}", folder);
        let output_path = if cfg!(windows) {
            folder.join(format!("{}.exe", TARGET_FILE_BASE_NAME))
        } else {
            folder.join(TARGET_FILE_BASE_NAME)
        };
        let runner = CompileRunner::new(&compiler, exo, &output_path)
            .ok_or(CompilationStartError::ErrorStartingCompileProcess)?;
        info!("Command: {:#?}", runner.get_full_command());
        App::start_work(wh, Box::new(runner))
            .ok_or(CompilationStartError::ErrorStartingCompileProcess)?;
        return Ok(output_path);
    }

    /// Cleans the previous run by stopping and waiting for every non UI worker to finish
    ///
    /// This function may block if a worker takes too long to finish
    ///
    pub(super) fn cleanup_previous_run(wh: &Arc<Mutex<WorkHandler>>) {
        //Clean up previous exo workers
        if let Ok(mut wh) = wh.lock() {
            wh.clean_non_ui_workers()
        }
    }

    /// Starts a new exo
    ///
    /// Starting an exercise essentially means doing 3 things:
    /// - Cleanup the previous run so data from the previous run doesn't interfere with the new one
    /// - Open the editor. See `open_editor` for more details
    /// - Compile. See `compile` for more details
    /// - Setup this exercise file watchers. See `start_watcher` for more details
    /// Every step but the first is done using a separate worker
    /// so this function doesn't block
    /// Returns a `ExoStatusReport` if the exo was successfully started
    /// else it returns an error
    pub(super) fn start_exo(
        wh: &Arc<Mutex<WorkHandler>>,
        exo: &Exo,
    ) -> Result<ExoStatusReport, StartExoFail> {
        App::cleanup_previous_run(wh);
        // Open editor, Compile exo and start watchers
        // TODO warn user if we couldn't open editor but ignore error for now so it doesn't stop us
        // from launching
        let _ = App::open_editor(wh, exo); // Ignore Error while opening editor for now
        let output_path =
            App::compile(wh, exo).map_err(|err| StartExoFail::CouldNotStartCompilation(err))?;
        App::start_watcher(wh, exo);

        Ok(ExoStatusReport::new(exo, output_path))
    }
    /// Runs the target file generated at the compilation step
    /// Here we launch multiple instances of the target file, one for each exo check
    ///
    /// This function doesn't block each instance of the target file will be launched using a separate worker
    pub(super) fn start_runners(&mut self) {
        if let Some(ref mut cr) = self.current_run {
            cr.check_results
                .iter_mut()
                .enumerate()
                .for_each(|(id, result)| {
                    if let Some(worker) =
                        Launcher::new(id, cr.elf_path.clone(), result.state.check.args.clone())
                    {
                        if App::start_work(&self.work_handler, Box::new(worker)).is_some() {
                            result.state.status = CheckStatus::Running;
                            result.output.clear();
                        }
                    }
                });
        }
    }

    /// Launches the specified check
    /// Warning: This function should only be called once the run is over
    /// else the check will probably fail
    ///
    pub(super) fn start_check(&mut self, id: usize) -> Option<usize> {
        if let Some(ref mut cr) = self.current_run {
            if id < cr.check_results.len() {
                let checker = Checker::new(
                    id,
                    Arc::clone(&cr.check_results[id].state.check),
                    cr.check_results[id].output.join("\n"),
                );
                return Some(App::start_work(&self.work_handler, Box::new(checker))?);
            }
        }
        None
    }

    /// Launches a file watcher for the current exo
    /// This function doesn't block
    /// Returns a vector containing the id of the workers that were successfully started
    pub(super) fn start_watcher(wh: &Arc<Mutex<WorkHandler>>, exo: &Exo) -> Vec<usize> {
        exo.files
            .iter()
            .filter_map(|file| {
                let watcher = FileWatcher::new(file.clone());
                if let Some(id) = App::start_work(wh, Box::new(watcher)) {
                    return Some(id);
                }
                None
            })
            .collect()
    }
}
