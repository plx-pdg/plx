use std::sync::{
    mpsc::{self, Receiver, Sender},
    Arc, Mutex,
};

use crate::models::{event::Event, exo::Exo, project::Project, ui_state::UiState};

use super::{
    core_error::CoreInitError,
    editor::opener::EditorOpener,
    file_utils::file_handler::current_folder,
    parser::from_dir::FromDir,
    work::{work::Work, work_handler::WorkHandler},
};

pub struct App {
    ui_state: UiState,
    project: Project,
    work_handler: Arc<Mutex<WorkHandler>>,
    event_queue: (Sender<Event>, Receiver<Event>),
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

        let channel = mpsc::channel();
        Ok(App {
            ui_state: UiState::Home,
            project,
            work_handler: (WorkHandler::new(channel.0.clone())),
            event_queue: channel,
        })
    }
    pub fn get_state(&self) -> &UiState {
        &self.ui_state
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
