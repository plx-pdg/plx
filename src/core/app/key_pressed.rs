use std::sync::Arc;

use crate::models::ui_state::UiState;

use super::app::App;

impl App {
    pub(super) fn on_r(&mut self) {
        match &self.state.ui_state {
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

    pub(super) fn on_j(&mut self) {
        match &self.state.ui_state {
            UiState::Home => todo!(),
            UiState::Quit => todo!(),
            UiState::Help => todo!(),
            UiState::SkillSelection { skill_index, .. } => self.next_skill(*skill_index),
            UiState::ExoSelection {
                skill_index,
                exo_index,
                ..
            }
            | UiState::ExoPreview {
                skill_index,
                exo_index,
                ..
            } => self.next_exo(*skill_index, *exo_index),

            UiState::CompileError { scroll_offset, .. }
            | UiState::CheckResults { scroll_offset, .. }
            | UiState::ExoDone { scroll_offset, .. }
            | UiState::ShowSolution { scroll_offset, .. } => self.scroll_down(*scroll_offset),
            _ => (),
        };
    }
    pub(super) fn on_h(&mut self) {
        match &self.state.ui_state {
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
            UiState::CompileError { .. } | UiState::CheckResults { .. } => {
                self.set_ui_state(UiState::ExoPreview {
                    skill_index: self.state.last_skill_index,
                    exo_index: self.state.last_exo_index,
                    skills: Arc::clone(&self.project.skills),
                    exo: Arc::clone(&self.state.last_exo),
                    exos: Arc::clone(&self.project.skills[self.state.last_skill_index].exos),
                })
            }
            UiState::ExoDone { .. } | UiState::ShowSolution { .. } => {
                self.set_ui_state(UiState::Compiling {
                    exo: Arc::clone(&self.state.last_exo),
                });
            }
            _ => {}
        }
    }
    pub(super) fn on_l(&mut self) {
        match &self.state.ui_state {
            UiState::SkillSelection { skill_index, .. } => {
                let exo_index =
                    if let Some((idx, _)) = self.project.skills[*skill_index].get_next_todo_exo() {
                        idx
                    } else {
                        0
                    };

                self.set_ui_state(UiState::ExoSelection {
                    skill_index: *skill_index,
                    skills: Arc::clone(&self.project.skills),
                    exos: Arc::clone(&self.project.skills[*skill_index].exos),
                    exo_index,
                })
            }
            UiState::ExoSelection {
                skill_index,
                exo_index,
                ..
            } => self.set_ui_state(UiState::ExoPreview {
                skill_index: *skill_index,
                skills: Arc::clone(&self.project.skills),
                exos: Arc::clone(&self.project.skills[*skill_index].exos),
                exo_index: *exo_index,
                exo: Arc::new(self.project.skills[*skill_index].exos[*exo_index].clone()),
            }),
            // UiState::ExoPreview {
            //     skill_index,
            //     exo_index,
            //     exos,
            //     skills,
            //     ..
            // } =>

            //     self.open_editor()
            //     self.start_compiling();
            // self.set_ui_state(UiState::ExoSelection {
            //         skill_index: *skill_index,
            //         exo_index: *exo_index,
            //         exos: exos.clone(),
            //         skills: skills.clone(),
            //     }),
            //     UiState::CompileError { .. } | UiState::CheckResults { .. } => {
            //         self.set_ui_state(UiState::ExoPreview {
            //             skill_index: self.state.last_skill_index,
            //             exo_index: self.state.last_exo_index,
            //             skills: Arc::clone(&self.project.skills),
            //             exo: Arc::clone(&self.state.last_exo),
            //             exos: Arc::clone(&self.project.skills[self.state.last_skill_index].exos),
            //         })
            //     }
            //     UiState::ExoDone { .. } | UiState::ShowSolution { .. } => {
            //         self.set_ui_state(UiState::Compiling {
            //             exo: Arc::clone(&self.state.last_exo),
            //         });
            //     }
            _ => {}
        }
    }
}
