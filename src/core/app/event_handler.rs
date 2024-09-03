use crate::{
    core::{check::output_checker::OutputChecker, diff::diff::Diff},
    models::{check::CheckTest, check_state::CheckStatus, key::Key},
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
        }
    }
    pub(super) fn on_process_creation_fail(&mut self) {
        //TODO warn user
    }
    pub(super) fn on_process_output_line(&mut self, run_idx: usize, line: String) {
        if let Some(ref mut cr) = self.current_run {
            if run_idx < cr.checkers.len() {
                cr.checkers[run_idx].output.push(line)
            }
        }
    }
    fn on_check_status(&mut self, check_idx: usize, check_status: CheckStatus) {
        if let Some(ref mut cr) = self.current_run {
            if check_idx < cr.checkers.len() {
                cr.checkers[check_idx].state.status = check_status;
            }
        }
    }
    pub(super) fn on_check_passed(&mut self, check_idx: usize) {
        self.on_check_status(check_idx, CheckStatus::Passed);
    }

    pub(super) fn on_check_failed(&mut self, check_idx: usize, diff: Diff) {
        if let Some(ref mut cr) = self.current_run {
            if check_idx < cr.checkers.len() {
                match &cr.checkers[check_idx].state.check.test {
                    CheckTest::Output { expected } => CheckStatus::Failed(
                        expected.clone(),
                        cr.checkers[check_idx].output.join("\n"),
                        diff,
                    ),
                };
            }
        }
    }
    pub(super) fn on_compilation_output(&mut self, line: String) {
        if let Some(ref mut cr) = self.current_run {
            cr.compilation_output.push(line);
        }
    }
    pub(super) fn on_compilation_end(&mut self, success: bool) {
        if success {
            self.start_runners();
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
            if id < cr.checkers.len() {
                cr.checkers[id].state.status = CheckStatus::Running;
            }
        }
    }
    pub(super) fn on_run_end(&mut self, id: usize) {
        if let Some(ref mut cr) = self.current_run {
            if id < cr.checkers.len() {
                cr.checkers[id].state.status = CheckStatus::Checking;
                self.start_check(id);
            }
        }
    }
    pub(super) fn on_run_output(&mut self, id: usize, line: String) {
        if let Some(ref mut cr) = self.current_run {
            if id < cr.checkers.len() {
                cr.checkers[id].output.push(line);
            }
        }
    }
}
