use crate::{
    core::diff::diff::Diff,
    models::{
        check::CheckTest, check_state::CheckStatus, exo_state::ExoState, project::Project,
        ui_state::UiState,
    },
};

use super::app::App;

/// Functions related to handling check updates
impl App {
    /// Check passed event handler
    /// Gets called when a check passes
    pub(super) fn on_check_passed(&mut self, check_idx: usize) {
        self.on_check_status(check_idx, CheckStatus::Passed);
    }

    /// Check failed event handler
    /// Gets called when a check fails
    pub(super) fn on_check_failed(&mut self, check_idx: usize, diff: Diff) {
        if let Some(ref mut cr) = self.current_run {
            if check_idx < cr.check_results.len() {
                match &cr.check_results[check_idx].state.check.test {
                    CheckTest::Output { expected } => {
                        let output = cr.check_results[check_idx].output.join("\n").clone();
                        let expected = expected.clone();

                        self.on_check_status(
                            check_idx,
                            CheckStatus::Failed(expected, output, diff),
                        );
                    }
                };
            }
        }
    }
    /// On check status change
    /// Utility function, should not be called directly.
    /// It refactors common code that should be done when the check status changes
    /// See `on_check_passed` and `on_check_failed` functions
    /// This function:
    ///  > Updates the status
    ///  > Updates the UI using `on_new_check_update`
    fn on_check_status(&mut self, check_idx: usize, check_status: CheckStatus) {
        if let Some(ref mut cr) = self.current_run {
            if check_idx < cr.check_results.len() {
                cr.check_results[check_idx].state.status = check_status;
            }
        }
        self.on_new_check_update();
    }

    /// on check update
    /// Utility function, should not be called directly.
    /// It is used to update the ui state and send it to the frontend
    /// See `on_check_passed` and `on_check_failed` functions
    /// This function updates the UI state
    fn on_new_check_update(&mut self) {
        if let Some(ref cr) = self.current_run {
            // Keep the same scroll offset if we're already checking the results
            let scroll_offset = match self.ui_state {
                UiState::CheckResults { scroll_offset, .. } => scroll_offset,
                _ => 0,
            };
            Project::set_exo_state(&cr.exo, ExoState::InProgress);
            self.go_to_check_results(scroll_offset, cr.to_vec_check_state());
        }
    }
}
