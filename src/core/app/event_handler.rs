use crate::{
    core::diff::diff::Diff,
    models::{check::CheckTest, check_state::CheckStatus, key::Key, ui_state::UiState},
};

use super::app::App;

impl App {
    pub(super) fn on_key_press(&mut self, key: Key) {
        match key {
            Key::Q => self.on_q(),
            Key::R => self.on_r(),
            Key::H => self.on_h(),
            Key::J => self.on_j(),
            Key::K => self.on_k(),
            Key::L | Key::Enter => self.on_l(),
            Key::N => todo!(),
            Key::P => todo!(),
            Key::E => todo!(),
            Key::Esc => self.on_esc(),
            Key::Interrogation => self.on_interrogation(Box::new(self.ui_state.clone()), 0),
        }
    }

    // Function to check if we're not already on the help page else we won't be able to exit.
    pub(super) fn on_interrogation(&mut self, last_state: Box<UiState>, scroll_offset: usize) {
        if !matches!(&self.ui_state, UiState::Help { .. }) {
            self.go_to_help(last_state, scroll_offset);
        }
    }

    pub(super) fn on_process_creation_fail(&mut self, run_id: usize, err: String) {
        if let Some(ref mut cr) = self.current_run {
            if run_id < cr.check_results.len() {
                cr.check_results[run_id].state.status = CheckStatus::RunFail(err);
            }
        }
    }
    pub(super) fn on_process_output_line(&mut self, run_id: usize, line: String) {
        if let Some(ref mut cr) = self.current_run {
            if run_id < cr.check_results.len() {
                cr.check_results[run_id].output.push(line)
            }
        }
    }
    fn on_check_status(&mut self, check_idx: usize, check_status: CheckStatus) {
        if let Some(ref mut cr) = self.current_run {
            if check_idx < cr.check_results.len() {
                cr.check_results[check_idx].state.status = check_status;
            }
        }
        self.on_new_check_update();
    }
    fn on_new_check_update(&mut self) {
        if let Some(ref cr) = self.current_run {
            let scroll_offset = match self.ui_state {
                UiState::CheckResults { scroll_offset, .. } => scroll_offset,
                _ => 0,
            };
            self.go_to_check_results(scroll_offset, cr.to_vec_check_state());
        }
    }
    pub(super) fn on_check_passed(&mut self, check_idx: usize) {
        self.on_check_status(check_idx, CheckStatus::Passed);
    }

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
    pub(super) fn on_run_start(&mut self, id: usize) {
        if let Some(ref mut cr) = self.current_run {
            if id < cr.check_results.len() {
                cr.check_results[id].state.status = CheckStatus::Running;
            }
        }
    }
    pub(super) fn on_run_end(&mut self, id: usize) {
        if let Some(ref mut cr) = self.current_run {
            if id < cr.check_results.len() {
                cr.check_results[id].state.status = CheckStatus::Checking;
                self.start_check(id);
            }
        }
    }
    pub(super) fn on_run_output(&mut self, id: usize, line: String) {
        if let Some(ref mut cr) = self.current_run {
            if id < cr.check_results.len() {
                cr.check_results[id].output.push(line);
            }
        }
    }
    pub(super) fn on_file_save(&mut self) {
        if let Some(ref mut cr) = self.current_run {
            App::compile(&self.work_handler, &cr.exo);
        }
    }
}
