use crate::models::{plx_project::PlxProject, ui_state::UiState};

use super::file_utils::file_handler;
pub struct PlxCore<'a> {
    ui_state: UiState<'a>,
    project: PlxProject,
}

impl PlxCore<'_> {
    pub fn new() -> Option<Self> {
        if !file_handler::is_plx_folder() {
            return None;
        }
        let project_file = file_handler::project_file();
        let project = PlxProject::try_from(project_file);
        if let Ok(project) = project {
            Some(PlxCore {
                ui_state: UiState::StartMenu,
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
