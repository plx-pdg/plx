use std::sync::{atomic::AtomicBool, mpsc::Sender, Arc};

use crate::{
    core::diff::difference::Difference,
    models::{check::Check, event::Event},
};

#[derive(Debug)]
pub enum OutputCheckerCreationError {
    InvalidCheck,
}
pub struct OutputChecker<'a> {
    check: &'a Check,
    program_output: &'a str,
}

impl<'a> OutputChecker<'a> {
    pub fn new(
        check: &'a Check,
        program_output: &'a str,
    ) -> Result<Self, OutputCheckerCreationError> {
        match check {
            Check::Output { .. } => Ok(Self {
                check,
                program_output,
            }),
            // _ => Err(OutputCheckerCreationError::InvalidCheck),
        }
    }
    pub fn run(&self, tx: Sender<Event>, _should_stop: Arc<AtomicBool>) {
        let expected = match self.check {
            Check::Output { expected } => expected,
            //_ => return, // Will never get here
        };

        let diff = Difference::calculate_difference(self.program_output, &expected, None);

        let event = if diff.contains_differences() {
            Event::OutputCheckFailed(diff)
        } else {
            Event::OutputCheckPassed
        };

        let _ = tx.send(event);
    }
}
