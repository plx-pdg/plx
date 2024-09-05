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
    pub(super) fn get_solution_file(
        exo: &Exo,
        solution_idx: usize,
    ) -> Result<std::path::PathBuf, ()> {
        // TODO create a more explicit error type
        if exo.solutions.is_empty() {
            return Err(());
        }

        if solution_idx >= exo.solutions.len() {
            return Err(());
        }
        return Ok(exo.solutions[solution_idx].clone());
    }
}
