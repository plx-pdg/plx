use std::sync::Arc;

use crate::models::{check_state::CheckState, ui_state::UiState};

use super::app::App;

impl App {
    pub(super) fn go_to_home(&mut self) {
        self.set_ui_state(UiState::Home);
    }
    pub(super) fn go_to_skill_selection(&mut self) {
        self.set_ui_state(UiState::SkillSelection {
            skill_index: self.project.state.curr_skill_idx,
            skills: Arc::clone(&self.project.skills),
            exos: Arc::clone(&self.project.skills[self.project.state.curr_skill_idx].exos),
        })
    }
    pub(super) fn go_to_exo_selection(&mut self) {
        self.set_ui_state(UiState::ExoSelection {
            skill_index: self.project.state.curr_skill_idx,
            exo_index: self.project.state.curr_exo_idx,
            skills: Arc::clone(&self.project.skills),
            exos: Arc::clone(&self.project.skills[self.project.state.curr_skill_idx].exos),
        })
    }
    pub(super) fn go_to_exo_preview(&mut self) {
        self.set_ui_state(UiState::ExoPreview {
            skill_index: self.project.state.curr_skill_idx,
            exo_index: self.project.state.curr_exo_idx,
            skills: Arc::clone(&self.project.skills),
            exos: Arc::clone(&self.project.skills[self.project.state.curr_skill_idx].exos),
            exo: Arc::new(
                self.project.skills[self.project.state.curr_skill_idx].exos
                    [self.project.state.curr_exo_idx]
                    .clone(),
            ),
        })
    }
    pub(super) fn go_to_compiling(&mut self) {
        self.set_ui_state(UiState::Compiling {
            exo: Arc::new(
                self.project.skills[self.project.state.curr_skill_idx].exos
                    [self.project.state.curr_exo_idx]
                    .clone(),
            ),
        })
    }
    pub(super) fn go_to_compilation_error(&mut self, scroll_offset: usize, error: String) {
        self.set_ui_state(UiState::CompileError {
            exo: Arc::new(
                self.project.skills[self.project.state.curr_skill_idx].exos
                    [self.project.state.curr_exo_idx]
                    .clone(),
            ),
            scroll_offset,
            error,
        })
    }
    pub(super) fn go_to_check_results(&mut self, scroll_offset: usize, checks: Vec<CheckState>) {
        self.set_ui_state(UiState::CheckResults {
            exo: Arc::new(
                self.project.skills[self.project.state.curr_skill_idx].exos
                    [self.project.state.curr_exo_idx]
                    .clone(),
            ),
            scroll_offset,
            checks,
        })
    }
    pub(super) fn go_to_solution(&mut self, scroll_offset: usize) {
        self.set_ui_state(UiState::ShowSolution {
            exo: Arc::new(
                self.project.skills[self.project.state.curr_skill_idx].exos
                    [self.project.state.curr_exo_idx]
                    .clone(),
            ),
            scroll_offset,
        })
    }
    pub(super) fn go_to_help(&mut self, last_state: Box<UiState>, scroll_offset: usize) {
        self.set_ui_state(UiState::Help {
            last_state,
            scroll_offset,
        })
    }
}
