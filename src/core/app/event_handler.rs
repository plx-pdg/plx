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
        self.on_check_status(check_idx, true);
    }

    pub(super) fn on_check_failed(&mut self, check_idx: usize) {
        self.on_check_status(check_idx, false);
    }
}
