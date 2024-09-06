use std::sync::Arc;

use crate::{
    core::work::{work::Work, work_type::WorkType},
    models::check::{Check, CheckTest},
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
    fn run(
        &self,
        tx: std::sync::mpsc::Sender<crate::models::event::Event>,
        stop: std::sync::Arc<std::sync::atomic::AtomicBool>,
    ) -> bool {
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
