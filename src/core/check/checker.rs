use std::sync::{atomic::AtomicBool, mpsc::Sender, Arc};

use crate::{
    core::work::{work::Work, work_type::WorkType},
    models::{
        check::{Check, CheckTest},
        event::Event,
    },
};

use super::output_checker::OutputChecker;

/// Checker struct, contains necessary data to run checks
/// Represents the checker worker
pub struct Checker {
    id: usize,
    check: Arc<Check>,
    program_output: String,
}

impl Checker {
    pub fn new(id: usize, check: Arc<Check>, program_output: String) -> Self {
        Self {
            id,
            check,
            program_output,
        }
    }
}
impl Work for Checker {
    /// Runs a check based on its type
    fn run(&self, tx: Sender<Event>, stop: Arc<AtomicBool>) -> bool {
        // Run dedicated checker based on check type
        match &self.check.test {
            CheckTest::Output { expected } => {
                let output_checker = OutputChecker::new(self.id, &self.program_output, &expected);
                output_checker.run(tx, stop);
            }
        }
        return true;
    }

    fn work_type(&self) -> WorkType {
        WorkType::Checker
    }
}
