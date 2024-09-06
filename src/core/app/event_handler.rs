use log::{error, warn};

use crate::{
    core::diff::diff::Diff,
    models::{
        check::CheckTest, check_state::CheckStatus, exo_state::ExoState, key::Key,
        project::Project, ui_state::UiState,
    },
};

use super::app::App;

impl App {
    /// Key press Event Handler
    pub(super) fn on_key_press(&mut self, key: Key) {
        match key {
            Key::Q => self.on_q(),
            Key::R => self.on_r(),
            Key::H => self.on_h(),
            Key::J => self.on_j(),
            Key::K => self.on_k(),
            Key::L | Key::Enter => self.on_l(),
            Key::N => self.on_n(),
            Key::P => self.on_p(),
            Key::E => {}
            Key::Esc => self.on_esc(),
            Key::Interrogation => self.on_interrogation(Box::new(self.ui_state.clone()), 0),
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
    /// Compilation output event handler
    /// Gets called when the compilation process outputs a new line
    pub(super) fn on_compilation_start(&mut self) {
        if let Some(ref mut cr) = self.current_run {
            cr.compilation_output.clear();
        }
        self.go_to_compiling();
    }
    pub(super) fn on_compilation_output(&mut self, line: String) {
        if let Some(ref mut cr) = self.current_run {
            cr.compilation_output.push(line);
        }
    }

    /// Compilation finished event handler
    /// Gets called when the target binary compilation ends
    pub(super) fn on_compilation_end(&mut self, success: bool) {
        if success {
            self.start_runners();
            if let Some(ref cr) = self.current_run {
                self.go_to_check_results(
                    0,
                    cr.check_results
                        .iter()
                        .map(|result| result.state.clone())
                        .collect(),
                );
            }
        } else {
            let output = if let Some(ref cr) = self.current_run {
                cr.compilation_output.join("\n")
            } else {
                String::from("")
            };
            self.go_to_compilation_error(0, output)
        }
    }

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
    /// File saved event handler
    /// Called when one the current exo files gets saved
    pub(super) fn on_file_save(&mut self) {
        if let Some(ref mut cr) = self.current_run {
            if let Err(err) = App::compile(&self.work_handler, &cr.exo) {
                error!("Error Starting Compilation {}", err);
            }
        }
    }
}
