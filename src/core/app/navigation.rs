use std::sync::Arc;

use crate::models::ui_state::UiState;

use super::app::App;

//Skills
impl App {
    pub(super) fn prev_skill(&mut self, skill_index: usize) {
        if skill_index > 0 {
            let skill_index = skill_index - 1;
            self.set_skill_selection(skill_index);
        };
    }
    pub(super) fn next_skill(&mut self, skill_index: usize) {
        if skill_index < self.project.skills.len() {
            let skill_index = skill_index + 1;
            self.set_skill_selection(skill_index);
        };
    }
    pub(super) fn set_skill_selection(&mut self, skill_index: usize) {
        self.state.last_skill_index = skill_index;
        self.set_ui_state(UiState::SkillSelection {
            skill_index,
            skills: self.project.skills.clone(),
            exos: self.project.skills[skill_index].exos.clone(),
        });
    }
}

//Exos
impl App {
    pub(super) fn prev_exo(&mut self, skill_index: usize, exo_index: usize) {
        if exo_index > 0 {
            let exo_index = exo_index - 1;
            self.set_exo_selection(skill_index, exo_index);
        };
    }
    pub(super) fn next_exo(&mut self, skill_index: usize, exo_index: usize) {
        if exo_index < self.project.skills[skill_index].exos.len() {
            let exo_index = exo_index + 1;
            self.set_exo_selection(skill_index, exo_index);
        };
    }
    pub(super) fn set_exo_selection(&mut self, skill_index: usize, exo_index: usize) {
        self.state.last_exo_index = exo_index;
        match self.state.ui_state {
            UiState::ExoSelection { .. } => {
                self.set_ui_state(UiState::ExoSelection {
                    skill_index,
                    exo_index,
                    skills: self.project.skills.clone(),
                    exos: self.project.skills[skill_index].exos.clone(),
                });
            }
            UiState::ExoPreview { .. } => {
                self.set_ui_state(UiState::ExoPreview {
                    skill_index,
                    exo_index,
                    skills: self.project.skills.clone(),
                    exos: self.project.skills[skill_index].exos.clone(),
                    exo: Arc::new(self.project.skills[skill_index].exos[exo_index].clone()),
                });
            }
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
