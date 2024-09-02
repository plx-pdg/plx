use std::sync::{
    mpsc::{self, Receiver, Sender},
    Arc, Mutex,
};

use crate::{
    models::{event::Event, exo::Exo, key::Key, project::Project, ui_state::UiState},
    ui::ui::Ui,
};

use super::{
    core_error::CoreInitError,
    editor::opener::EditorOpener,
    file_utils::file_handler::current_folder,
    parser::from_dir::FromDir,
    work::{work::Work, work_handler::WorkHandler, work_type::WorkType},
};

pub struct App {
    ui_state: UiState,
    project: Project,
    work_handler: Arc<Mutex<WorkHandler>>,
    event_tx: Sender<Event>,
    event_rx: Receiver<Event>,
    ui_state_tx: Sender<UiState>,
    run: bool,
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
        };
        app.start_ui(ui_state_rx);
        Ok(app)
    }
    pub fn run_forever(mut self) {
        while self.run {
            if let Ok(event) = self.event_rx.recv() {
                match event {
                    Event::KeyPressed(Key::Q) => {
                        self.run = false;
                        if let Ok(mut wh) = self.work_handler.lock() {
                            wh.stop_all_workers_and_wait();
                        }
                    }
                    Event::KeyPressed(_) => todo!(),
                    Event::EditorOpened => todo!(),
                    Event::CouldNotOpenEditor => todo!(),
                    Event::ProcessCreationFailed => todo!(),
                    Event::ProcessOutputLine(_) => todo!(),
                    Event::OutputCheckPassed(_) => todo!(),
                    Event::OutputCheckFailed(_, _) => todo!(),
                }
            }
        }
    }
    fn start_work(&mut self, work: Box<dyn Work + Send>) {
        if let Ok(mut work_handler) = self.work_handler.lock() {
            work_handler.spawn_worker(work);
        }
    }
    fn start_ui(&mut self, ui_state_rx: Receiver<UiState>) {
        let ui = Ui::new(ui_state_rx);
        let _ = self.ui_state_tx.send(UiState::Home);
        self.start_work(Box::new(ui));
    }
    fn stop_ui(&mut self) {
        if let Ok(mut work_handler) = self.work_handler.lock() {
            work_handler.stop_workers(WorkType::Ui);
        }
    }

    fn open_editor(&mut self, exo: &Exo) {
        if let Some(file) = exo.get_main_file() {
            if let Some(opener) = EditorOpener::new_default_editor(file.to_path_buf()) {
                self.start_work(Box::new(opener));
            }
        }
    }
}
