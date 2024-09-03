use crate::models::exo::Exo;

use super::app::App;

impl App {
    pub(super) fn current_exo(&self) -> &Exo {
        &self.project.skills[self.project.state.curr_skill_idx].exos
            [self.project.state.curr_exo_idx]
    }
}
