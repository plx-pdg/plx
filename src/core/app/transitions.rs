use std::{path::PathBuf, sync::Arc};

use crate::{
    core::file_utils::file_utils::read_file,
    models::{check_state::CheckState, ui_state::UiState},
};

use super::app::App;


/// Helper functions to refactor setting the ui state
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
    pub(super) fn go_to_solution(&mut self, scroll_offset: usize, solution_idx: usize) {
        let exo = self.current_exo();

        let state = if let Ok(solution_path) = App::get_solution_file(exo, 0) {
            if let Ok(solution_content) = read_file(&solution_path) {
                UiState::ShowSolution {
                    exo: Arc::new(exo.clone()),
                    solution: solution_content,
                    solution_idx,
                    solution_path,
                    scroll_offset,
                }
            } else {
                UiState::ShowSolution {
                    exo: Arc::new(exo.clone()),
                    solution: String::from("Couldn't Read Solution File"),
                    solution_idx,
                    solution_path,
                    scroll_offset,
                }
            }
        } else {
            UiState::ShowSolution {
                exo: Arc::new(exo.clone()),
                solution: String::from("No Solution Found"),
                solution_idx,
                solution_path: PathBuf::new(),
                scroll_offset,
            }
        };
        self.set_ui_state(state);
    }
    pub(super) fn go_to_help(&mut self, last_state: Box<UiState>, scroll_offset: usize) {
        self.set_ui_state(UiState::Help {
            last_state,
            scroll_offset,
        })
    }
}
