use crate::models::ui_state::UiState;

use super::app::App;

impl App {
    pub(super) fn on_r(&mut self) {
        match &self.ui_state {
            UiState::Home => self.resume_last_exo(),
            _ => (),
        }
    }
    pub(super) fn on_q(&mut self) {
        self.set_ui_state(UiState::Quit);
        if let Ok(mut wh) = self.work_handler.lock() {
            wh.stop_all_workers_and_wait();
        }
    }

    pub(super) fn on_esc(&mut self) {
        match &self.ui_state {
            UiState::Help { last_state } => self.set_ui_state(*last_state.clone()),
            UiState::ExoPreview { .. } => self.go_to_exo_selection(),
            _ => {}
        }
    }

    pub(super) fn on_j(&mut self) {
        match &self.ui_state {
            UiState::SkillSelection { .. } => self.next_skill(),
            UiState::ExoSelection { .. } | UiState::ExoPreview { .. } => self.next_exo(false),

            UiState::CompileError { scroll_offset, .. }
            | UiState::CheckResults { scroll_offset, .. }
            | UiState::ExoDone { scroll_offset, .. }
            | UiState::ShowSolution { scroll_offset, .. } => self.scroll_down(*scroll_offset),
            _ => (),
        };
    }
    pub(super) fn on_k(&mut self) {
        match &self.ui_state {
            UiState::SkillSelection { .. } => self.prev_skill(),
            UiState::ExoSelection { .. } | UiState::ExoPreview { .. } => self.prev_exo(),

            UiState::CompileError { scroll_offset, .. }
            | UiState::CheckResults { scroll_offset, .. }
            | UiState::ExoDone { scroll_offset, .. }
            | UiState::ShowSolution { scroll_offset, .. } => self.scroll_up(*scroll_offset),
            _ => (),
        };
    }
    pub(super) fn on_h(&mut self) {
        match &self.ui_state {
            UiState::SkillSelection { .. } => self.set_ui_state(UiState::Home),
            UiState::ExoSelection {
                skill_index,
                skills,
                exos,
                ..
            } => self.set_ui_state(UiState::SkillSelection {
                skill_index: *skill_index,
                skills: skills.clone(),
                exos: exos.clone(),
            }),
            UiState::ExoPreview {
                skill_index,
                exo_index,
                exos,
                skills,
                ..
            } => self.set_ui_state(UiState::ExoSelection {
                skill_index: *skill_index,
                exo_index: *exo_index,
                exos: exos.clone(),
                skills: skills.clone(),
            }),
            UiState::CompileError { .. } | UiState::CheckResults { .. } => self.go_to_exo_preview(),
            UiState::ShowSolution { .. } => self.go_to_exo_done(0),
            _ => {}
        }
    }
    pub(super) fn on_l(&mut self) {
        match &self.ui_state {
            UiState::Home { .. } => self.go_to_skill_selection(),
            UiState::SkillSelection { .. } => self.go_to_exo_selection(),
            UiState::ExoSelection { .. } => self.go_to_exo_preview(),
            UiState::ExoPreview { exo, .. } => {
                App::start_exo(&self.work_handler, &exo);
                self.go_to_compiling();
            }
            UiState::ExoDone { .. } => self.go_to_solution(0),
            UiState::ShowSolution { .. } => {
                self.next_exo(true);
                let exo = &self.project.skills[self.project.state.curr_skill_idx].exos
                    [self.project.state.curr_exo_idx];
                App::start_exo(&self.work_handler, exo);
            }
            _ => {}
        }
    }
}
