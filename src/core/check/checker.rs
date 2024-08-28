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

#[cfg(test)]
mod test {

    use std::sync::mpsc::channel;

    use super::*;
    #[test]
    fn test_same_output() {
        let check = Check::Output {
            expected: String::from("hello"),
        };
        let output = "hello";
        let (tx, rx) = channel();

        let checker = OutputChecker::new(&check, output).unwrap();
        checker.run(tx, Arc::new(AtomicBool::new(false)));

        assert_eq!(rx.recv().unwrap(), Event::OutputCheckPassed);
    }

    #[test]
    fn test_diff_output() {
        let check = Check::Output {
            expected: String::from("hello"),
        };
        let output = "hello world";
        let (tx, rx) = channel();

        let checker = OutputChecker::new(&check, output).unwrap();
        checker.run(tx, Arc::new(AtomicBool::new(false)));
        matches!(rx.recv().unwrap(), Event::OutputCheckFailed(_));
    }

    #[test]
    fn test_extra_whitespace_at_the_end_is_equal() {
        let check = Check::Output {
            expected: String::from("hello"),
        };
        let output = "hello ";
        let (tx, rx) = channel();

        let checker = OutputChecker::new(&check, output).unwrap();
        checker.run(tx, Arc::new(AtomicBool::new(false)));
        matches!(rx.recv().unwrap(), Event::OutputCheckPassed);
    }

    #[test]
    fn test_extra_whitespace_beggining_is_different() {
        let check = Check::Output {
            expected: String::from("hello"),
        };
        let output = " hello";
        let (tx, rx) = channel();

        let checker = OutputChecker::new(&check, output).unwrap();
        checker.run(tx, Arc::new(AtomicBool::new(false)));
        matches!(rx.recv().unwrap(), Event::OutputCheckFailed(_));
    }

    #[test]
    fn test_tab_at_the_end_is_equal() {
        let check = Check::Output {
            expected: String::from("hello"),
        };
        let output = "hello\t";
        let (tx, rx) = channel();

        let checker = OutputChecker::new(&check, output).unwrap();
        checker.run(tx, Arc::new(AtomicBool::new(false)));
        matches!(rx.recv().unwrap(), Event::OutputCheckPassed);
    }
    #[test]
    fn test_new_line_at_the_end_is_equal() {
        let check = Check::Output {
            expected: String::from("hello"),
        };
        let output = "hello\n";
        let (tx, rx) = channel();

        let checker = OutputChecker::new(&check, output).unwrap();
        checker.run(tx, Arc::new(AtomicBool::new(false)));
        matches!(rx.recv().unwrap(), Event::OutputCheckPassed);
    }
}
