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
        self.run = false;
    }

    pub(super) fn on_esc(&mut self) {
        match &self.ui_state {
            UiState::Help { last_state, .. } => self.set_ui_state(*last_state.clone()),
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
            | UiState::ShowSolution { scroll_offset, .. }
            | UiState::Help { scroll_offset, .. } => self.scroll_down(*scroll_offset),
            _ => (),
        };
    }
    pub(super) fn on_k(&mut self) {
        match &self.ui_state {
            UiState::SkillSelection { .. } => self.prev_skill(),
            UiState::ExoSelection { .. } | UiState::ExoPreview { .. } => self.prev_exo(),

            UiState::CompileError { scroll_offset, .. }
            | UiState::CheckResults { scroll_offset, .. }
            | UiState::ShowSolution { scroll_offset, .. }
            | UiState::Help { scroll_offset, .. } => self.scroll_up(*scroll_offset),
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
            UiState::ShowSolution { .. } => {
                if let Some(cr) = &self.current_run {
                    self.go_to_check_results(0, cr.to_vec_check_state())
                }
            }
            _ => {}
        }
    }
    pub(super) fn on_l(&mut self) {
        match &self.ui_state {
            UiState::Home { .. } => self.go_to_skill_selection(),
            UiState::SkillSelection { .. } => self.go_to_exo_selection(),
            UiState::ExoSelection { .. } => self.go_to_exo_preview(),
            UiState::ExoPreview { exo, .. } => {
                self.current_run = App::start_exo(&self.work_handler, exo).ok();
                self.go_to_compiling();
            }
            UiState::CheckResults { checks, .. } => {
                if App::all_checks_passed(checks) {
                    self.go_to_solution(0, 0);
                }
            }
            UiState::ShowSolution { .. } => {
                self.next_exo(true);
                self.current_run = App::start_exo(&self.work_handler, self.current_exo()).ok();
            }
            _ => {}
        }
    }
    pub(super) fn on_p(&mut self) {
        match &self.ui_state {
            UiState::ShowSolution {
                scroll_offset,
                solution_idx,
                ..
            } => {
                if *solution_idx > 0 {
                    let solution_idx = *solution_idx - 1;
                    self.go_to_solution(*scroll_offset, solution_idx);
                }
            }

            UiState::CheckResults { checks, .. } => {
                if App::all_checks_passed(checks) {
                    self.go_to_solution(0, 0);
                }
            }
            _ => {}
        }
    }
    pub(super) fn on_n(&mut self) {
        match &self.ui_state {
            UiState::ShowSolution {
                scroll_offset,
                solution_idx,
                ..
            } => {
                let exo = self.current_exo();
                if *solution_idx < exo.solutions.len() - 1 {
                    let solution_idx = *solution_idx + 1;
                    self.go_to_solution(*scroll_offset, solution_idx);
                }
            }

            UiState::CheckResults { checks, .. } => {
                if App::all_checks_passed(checks) {
                    self.go_to_solution(0, 0);
                }
            }
            _ => {}
        }
    }
}
