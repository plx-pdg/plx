use crate::models::check_state::CheckStatus;

use super::app::App;

impl App {
    /// Run start event handler
    /// Called when the compiled target is launched
    pub(super) fn on_run_start(&mut self, id: usize) {
        if let Some(ref mut cr) = self.current_run {
            if id < cr.check_results.len() {
                cr.check_results[id].state.status = CheckStatus::Running;
            }
        }
    }
    /// Run end event handler
    /// Called when the compiled target finishes execution
    pub(super) fn on_run_end(&mut self, id: usize) {
        if let Some(ref mut cr) = self.current_run {
            if id < cr.check_results.len() {
                cr.check_results[id].state.status = CheckStatus::Checking;
                self.start_check(id);
            }
        }
    }
    /// Run output event handler
    /// Called when the compiled target outputs a new line
    pub(super) fn on_run_output(&mut self, id: usize, line: String) {
        if let Some(ref mut cr) = self.current_run {
            if id < cr.check_results.len() {
                cr.check_results[id].output.push(line);
            }
        }
    }
    /// Run fail event handler
    /// Called when we were not able to launch the compiled target
    pub(super) fn on_run_fail(&mut self, run_id: usize, err: String) {
        if let Some(ref mut cr) = self.current_run {
            if run_id < cr.check_results.len() {
                cr.check_results[run_id].state.status = CheckStatus::RunFail(err);
            }
        }
    }
}
