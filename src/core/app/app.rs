use std::{
    path::PathBuf,
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
};

use crate::{
    core::{
        check::checker::Checker,
        compiler::compile_runner::CompileRunner,
        core_error::CoreInitError,
        editor::opener::EditorOpener,
        file_utils::{build_folder::generate_build_folder, file_handler::current_folder},
        launcher::launcher::Launcher,
        parser::from_dir::FromDir,
        watcher::watcher::FileWatcher,
        work::{work::Work, work_handler::WorkHandler, work_type::WorkType},
    },
    models::{
        check::Check,
        check_state::{CheckState, CheckStatus},
        constants::TARGET_FILE_BASE_NAME,
        event::Event,
        exo::Exo,
        project::Project,
        ui_state::UiState,
    },
    ui::ui::Ui,
};

pub(super) struct ExoCheckResult {
    pub(super) state: CheckState,
    pub(super) output: Vec<String>,
}
pub(super) struct ExoStatusReport {
    pub(super) check_results: Vec<ExoCheckResult>,
    pub(super) compilation_output: Vec<String>,
    pub(super) elf_path: PathBuf,
    pub(super) exo: Arc<Exo>,
}

impl ExoCheckResult {
    pub(super) fn new(check: &Check) -> Self {
        Self {
            state: CheckState::new(check),
            output: Vec::new(),
        }
    }
}
impl ExoStatusReport {
    pub(super) fn new(exo: &Exo, elf_path: PathBuf) -> Self {
        let checkers: Vec<ExoCheckResult> = exo
            .checks
            .iter()
            .map(|check| ExoCheckResult::new(check))
            .collect();

        Self {
            check_results: checkers,
            compilation_output: Vec::new(),
            elf_path,
            exo: Arc::new(exo.clone()),
        }
    }
}

pub struct App {
    pub(super) ui_state: UiState,
    pub(super) project: Project,
    pub(super) work_handler: Arc<Mutex<WorkHandler>>,
    pub(super) event_tx: Sender<Event>,
    pub(super) event_rx: Receiver<Event>,
    pub(super) ui_state_tx: Sender<UiState>,
    pub(super) run: bool,
    pub(super) current_run: Option<ExoStatusReport>,
}

impl App {
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
            event_tx,
            event_rx,
            ui_state_tx,
            run: true,
            current_run: None,
        };
        app.start_ui(ui_state_rx);
        Ok(app)
    }

    pub(super) fn set_ui_state(&mut self, new_state: UiState) {
        self.ui_state_tx.send(new_state.clone());
        self.ui_state = new_state;
    }
    pub(super) fn resume_last_exo(&mut self) {
        if let Some(exo) = &self.project.resume() {
            self.current_run = App::start_exo(&self.work_handler, exo).ok();
        } else {
            self.go_to_skill_selection();
        }
    }
    pub fn run_forever(mut self) {
        while self.run {
            if let Ok(event) = self.event_rx.recv() {
                println!("{:?}", event);
                match event {
                    Event::KeyPressed(key) => self.on_key_press(key),
                    Event::EditorOpened => {}
                    Event::CouldNotOpenEditor => {} //TODO warn the user ?
                    Event::ProcessOutputLine(run_id, line) => {
                        self.on_process_output_line(run_id, line)
                    }
                    Event::OutputCheckPassed(check_index) => self.on_check_passed(check_index),
                    Event::OutputCheckFailed(check_index, diff) => {
                        println!("{}", diff.to_ansi_colors());
                        self.on_check_failed(check_index, diff)
                    }
                    Event::FileSaved => self.on_file_save(),
                    Event::CompilationStart => self.go_to_compiling(),
                    Event::CompilationEnd(success) => self.on_compilation_end(success),
                    Event::CompilationOutputLine(line) => self.on_compilation_output(line),
                    Event::RunStart(id) => self.on_run_start(id),
                    Event::RunEnd(id) => self.on_run_end(id),
                    Event::RunOutputLine(id, line) => self.on_run_output(id, line),
                    Event::RunFail(run_id, err) => self.on_process_creation_fail(run_id, err),
                }
            }
        }
    }
    fn start_work(wh: &Arc<Mutex<WorkHandler>>, work: Box<dyn Work + Send>) -> Option<usize> {
        if let Ok(mut wh) = wh.lock() {
            return Some(wh.spawn_worker(work));
        }
        None
    }
    fn start_ui(&mut self, ui_state_rx: Receiver<UiState>) {
        let ui = Ui::new(ui_state_rx);
        let _ = self.ui_state_tx.send(UiState::Home);
        App::start_work(&self.work_handler, Box::new(ui));
    }
    fn stop_ui(&mut self) {
        if let Ok(mut work_handler) = self.work_handler.lock() {
            work_handler.stop_workers(WorkType::Ui);
        }
    }

    fn open_editor(wh: &Arc<Mutex<WorkHandler>>, exo: &Exo) -> Option<usize> {
        if let Some(file) = exo.get_main_file() {
            if let Some(opener) = EditorOpener::new_default_editor(file.to_path_buf()) {
                return App::start_work(wh, Box::new(opener));
            }
        }
        None
    }
    pub(super) fn compile(wh: &Arc<Mutex<WorkHandler>>, exo: &Exo) -> Option<PathBuf> {
        let compiler = exo.compiler()?;
        let folder = generate_build_folder(exo).ok()?;
        let output_path = if cfg!(windows) {
            folder.join(format!("{}.exe", TARGET_FILE_BASE_NAME))
        } else {
            folder.join(TARGET_FILE_BASE_NAME)
        };
        let runner = CompileRunner::new(&compiler, exo, &output_path)?;
        App::start_work(wh, Box::new(runner))?;
        return Some(output_path);
    }
    pub(super) fn cleanup_previous_run(wh: &Arc<Mutex<WorkHandler>>) {
        //Clean up previous exo workers
        if let Ok(mut wh) = wh.lock() {
            wh.clean_non_ui_workers()
        }
    }
    pub(super) fn start_exo(
        wh: &Arc<Mutex<WorkHandler>>,
        exo: &Exo,
    ) -> Result<ExoStatusReport, ()> {
        App::cleanup_previous_run(wh);
        // Open editor, Compile exo and start watchers
        App::open_editor(wh, exo).ok_or(())?;
        let output_path = App::compile(wh, exo).ok_or(())?;
        App::start_watcher(wh, exo);

        Ok(ExoStatusReport::new(exo, output_path))
    }
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

    pub(super) fn start_check(&mut self, id: usize) -> Option<usize> {
        if let Some(ref mut cr) = self.current_run {
            if id < cr.check_results.len() {
                // println!("Check: {}", cr.check_results[id].output.join("\n"));
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
