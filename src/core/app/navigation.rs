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
            self.set_scroll_offset(scroll_offset - 1);
        };
    }
    pub(super) fn scroll_down(&mut self, scroll_offset: usize) {
        self.set_scroll_offset(scroll_offset + 1);
    }
    pub(super) fn set_scroll_offset(&mut self, scroll_offset: usize) {
        match &self.ui_state {
            UiState::Help { last_state, .. } => self.go_to_help(last_state.clone(), scroll_offset),
            UiState::CompileError { error, .. } => {
                self.go_to_compilation_error(scroll_offset, error.to_string())
            }
            UiState::CheckResults { checks, .. } => {
                self.go_to_check_results(scroll_offset, checks.clone())
            }
            UiState::ExoDone { .. } => self.go_to_exo_done(scroll_offset),
            UiState::ShowSolution { .. } => self.go_to_solution(scroll_offset),
            _ => {}
        }
    }
}
