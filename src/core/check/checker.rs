use std::sync::{atomic::AtomicBool, mpsc::Sender, Arc};

use crate::models::{check::Check, event::Event};

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

    pub fn run(tx: Sender<Event>, should_stop: Arc<AtomicBool>) {}
}
