use std::sync::{
    mpsc::{self, Receiver, Sender},
    Arc, Mutex,
};

use crate::models::{event::Event, exo::Exo, project::Project, ui_state::UiState};

use super::{
    editor::opener::EditorOpener,
    file_utils::file_handler,
    parser,
    work::{work::Work, work_handler::WorkHandler},
};
pub struct App<'a> {
    ui_state: UiState<'a>,
    project: Project,
    work_handler: Arc<Mutex<WorkHandler>>,
    event_queue: (Sender<Event>, Receiver<Event>),
}

impl App<'_> {
    pub fn new() -> Option<Self> {
        if !file_handler::is_plx_folder() {
            return None;
        }
        let project_file = file_handler::project_file();

        let project = parser::object_creator::create_from_file(&project_file);
        if let Ok(project) = project {
            let channel = mpsc::channel();
            Some(App {
                ui_state: UiState::Home,
                project,
                work_handler: (WorkHandler::new(channel.0.clone())),
                event_queue: channel,
            })
        } else {
            None
        }
    }
    pub fn get_state(&self) -> &UiState {
        &self.ui_state
    }
    fn start_work(&mut self, work: Work) {
        if let Ok(mut work_handler) = self.work_handler.lock() {
            work_handler.spawn_worker(work);
        }
    }
    fn open_editor(&mut self, exo: &Exo) {
        if let Some(file) = exo.get_main_file() {
            if let Some(opener) = EditorOpener::new_default_editor(file.to_path_buf()) {
                self.start_work(Work::EditorOpen(opener));
            }
        }
    }
}
