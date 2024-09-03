use std::sync::Arc;

use crate::models::ui_state::UiState;

use super::app::App;

//Skills
impl App {
    pub(super) fn prev_skill(&mut self) {
        self.project.prev_skill(false);
        self.go_to_skill_selection();
    }
    pub(super) fn next_skill(&mut self) {
        self.project.next_skill(false);
        self.go_to_skill_selection();
    }
}

//Exos
impl App {
    pub(super) fn prev_exo(&mut self) {
        self.project.prev_exo(false);
        self.change_exo();
    }
    pub(super) fn next_exo(&mut self, wrap: bool) {
        self.project.next_exo(wrap);
        self.change_exo();
    }
    pub(super) fn change_exo(&mut self) {
        match self.ui_state {
            UiState::ExoSelection { .. } => self.go_to_exo_selection(),
            UiState::ExoPreview { .. } => self.go_to_exo_preview(),
            UiState::ShowSolution { .. } => self.go_to_compiling(),
            _ => {}
        };
    }
}

//Scroll
impl App {
    pub(super) fn scroll_up(&mut self, scroll_offset: usize) {
        if scroll_offset > 0 {
            let scroll_offset = scroll_offset - 1;
            self.set_scroll_offset(scroll_offset);
        };
    }
    pub(super) fn scroll_down(&mut self, scroll_offset: usize) {
        self.set_scroll_offset(scroll_offset + 1);
    }
    pub(super) fn set_scroll_offset(&mut self, scroll_offset: usize) {
        let state = match &self.state.ui_state {
            UiState::CompileError { exo, error, .. } => UiState::CompileError {
                scroll_offset,
                exo: exo.clone(),
                error: error.clone(),
            },
            UiState::CheckResults { exo, checks, .. } => UiState::CheckResults {
                scroll_offset,
                exo: exo.clone(),
                checks: checks.clone(),
            },
            UiState::ExoDone { exo, .. } => UiState::ExoDone {
                scroll_offset,
                exo: exo.clone(),
            },
            UiState::ShowSolution { exo, .. } => UiState::ShowSolution {
                scroll_offset,
                exo: exo.clone(),
            },
            _ => return,
        };
        self.set_ui_state(state);
    }
}

//Left
impl App {
    pub(super) fn last_page(&mut self) {}
}
