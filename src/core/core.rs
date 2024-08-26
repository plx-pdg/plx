use crate::models::{project::Project, ui_state::UiState};

use super::file_utils::file_handler;
pub struct PlxCore<'a> {
    ui_state: UiState<'a>,
    project: Project,
}

impl PlxCore<'_> {
    pub fn new() -> Option<Self> {
        if !file_handler::is_plx_folder() {
            return None;
        }
        let project_file = file_handler::project_file();
        let project = Project::try_from(project_file);
        if let Ok(project) = project {
            Some(PlxCore {
                ui_state: UiState::Home,
                project,
            })
        } else {
            None
        }
    }
    pub fn get_state(&self) -> &UiState {
        &self.ui_state
    }
}
