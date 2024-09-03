use crate::models::{
    check_state::{CheckState, CheckStatus},
    exo::Exo,
};

use super::app::App;

impl App {
    pub(super) fn current_exo(&self) -> &Exo {
        &self.project.skills[self.project.state.curr_skill_idx].exos
            [self.project.state.curr_exo_idx]
    }
    pub(super) fn all_checks_passed(checks: &Vec<CheckState>) -> bool {
        checks
            .iter()
            .all(|result| result.status == CheckStatus::Passed)
    }
}
