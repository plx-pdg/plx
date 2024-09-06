use log::error;

use crate::models::{exo_state::ExoState, project::Project, ui_state::UiState};

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

            //TODO refactor this code (duplicate)
            UiState::ExoPreview { exo, .. } => match App::start_exo(&self.work_handler, exo) {
                Ok(cr) => {
                    self.current_run = Some(cr);
                    self.go_to_compiling();
                }
                Err(err) =>
                //TODO send this to the ui
                {
                    error!("Could not launch exo {}", err);
                }
            },
            UiState::CheckResults { checks, exo, .. } => {
                if App::all_checks_passed(checks) {
                    Project::set_exo_state(exo, ExoState::Done);
                    self.go_to_solution(0, 0);
                }
            }
            UiState::ShowSolution { .. } => {
                self.next_exo(true);

                //TODO refactor this code (duplicate)
                match App::start_exo(&self.work_handler, self.current_exo()) {
                    Ok(cr) => {
                        self.current_run = Some(cr);
                        self.go_to_compiling();
                    }
                    Err(err) =>
                    //TODO send this to the ui
                    {
                        error!("Could not launch exo {}", err);
                    }
                }
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

            UiState::CheckResults { checks, exo, .. } => {
                if App::all_checks_passed(checks) {
                    Project::set_exo_state(exo, ExoState::Done);
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

            UiState::CheckResults { checks, exo, .. } => {
                if App::all_checks_passed(checks) {
                    Project::set_exo_state(exo, ExoState::Done);
                    self.go_to_solution(0, 0);
                }
            }
            _ => {}
        }
    }
    ///
    /// Handles the '?' key press
    pub(super) fn on_interrogation(&mut self, last_state: Box<UiState>, scroll_offset: usize) {
        if !matches!(&self.ui_state, UiState::Help { .. }) {
            self.go_to_help(last_state, scroll_offset);
        }
    }
}
