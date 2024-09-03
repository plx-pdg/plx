use std::sync::{atomic::AtomicBool, mpsc::Sender, Arc};

use crate::{
    core::{
        diff::diff::Diff,
        work::{work::Work, work_type::WorkType},
    },
    models::{
        check::{Check, CheckTest},
        event::Event,
    },
};

#[derive(Debug)]
pub enum OutputCheckerCreationError {
    InvalidCheck,
}
pub struct OutputChecker<'a> {
    id: usize,
    check: &'a Check,
    program_output: String,
}

impl<'a> OutputChecker<'a> {
    pub fn new(
        id: usize,
        check: &'a Check,
        program_output: String,
    ) -> Result<Self, OutputCheckerCreationError> {
        match check.test {
            CheckTest::Output { .. } => Ok(Self {
                id,
                check,
                program_output,
            }),
            // _ => Err(OutputCheckerCreationError::InvalidCheck),
        }
    }
}
impl Work for OutputChecker<'_> {
    fn run(&self, tx: Sender<Event>, _stop: Arc<AtomicBool>) -> bool {
        let expected = match &self.check.test {
            CheckTest::Output { expected } => expected,
            //_ => return, // Will never get here
        };

        let diff = Diff::calculate_difference(&self.program_output, expected, None);

        let event = if diff.contains_differences() {
            Event::OutputCheckFailed(self.id, diff)
        } else {
            Event::OutputCheckPassed(self.id)
        };

        let _ = tx.send(event);
        return true;
    }

    fn work_type(&self) -> WorkType {
        WorkType::OutputChecker
    }
}

#[cfg(test)]
mod test {

    use core::panic;
    use std::sync::mpsc::channel;

    use super::*;
    fn test(check: Check, output: &str) -> Event {
        let (tx, rx) = channel();

        let checker = OutputChecker::new(0, &check, output.to_string()).unwrap();
        checker.run(tx, Arc::new(AtomicBool::new(false)));

        rx.recv().unwrap()
    }
    #[test]
    fn test_id() {
        let check = Check {
            test: CheckTest::Output {
                expected: String::from("hello"),
            },
            name: String::new(),
            args: vec![],
        };
        let output = "hello";
        let (tx, rx) = channel();

        let checker = OutputChecker::new(0, &check, output.to_string()).unwrap();
        checker.run(tx, Arc::new(AtomicBool::new(false)));

        let id = match rx.recv().unwrap() {
            Event::OutputCheckPassed(id) => id,
            Event::OutputCheckFailed(id, _) => id,
            _ => panic!("Wrong event received"),
        };

        assert_eq!(id, 0);
    }
    #[test]
    fn test_same_output() {
        let check = Check {
            test: CheckTest::Output {
                expected: String::from("hello"),
            },
            name: String::new(),
            args: vec![],
        };
        let output = "hello";
        let event = test(check, output);
        assert!(matches!(event, Event::OutputCheckPassed(_)));
    }

    #[test]
    fn test_extra_word_at_the_end_fails() {
        let check = Check {
            test: CheckTest::Output {
                expected: String::from("hello"),
            },
            name: String::new(),
            args: vec![],
        };
        let output = "hello world";

        let event = test(check, output);
        assert!(matches!(event, Event::OutputCheckFailed(..)));
    }

    #[test]
    fn test_extra_whitespace_at_the_end_passes() {
        let check = Check {
            test: CheckTest::Output {
                expected: String::from("hello"),
            },
            name: String::new(),
            args: vec![],
        };
        let output = "hello ";

        let event = test(check, output);
        assert!(matches!(event, Event::OutputCheckPassed(..)));
    }

    #[test]
    fn test_extra_whitespace_beggining_fails() {
        let check = Check {
            test: CheckTest::Output {
                expected: String::from("hey\nhello"),
            },
            name: String::new(),
            args: vec![],
        };
        let output = "hey\n hello";
        let event = test(check, output);
        assert!(matches!(event, Event::OutputCheckFailed(..)));
    }

    #[test]
    fn test_tab_at_the_end_passes() {
        let check = Check {
            test: CheckTest::Output {
                expected: String::from("yoo\nhello"),
            },
            name: String::new(),
            args: vec![],
        };
        let output = "yoo\t\nhello\t";

        let event = test(check, output);
        assert!(matches!(event, Event::OutputCheckPassed(..)));
    }

    #[test]
    fn test_mix_of_whitespaces_at_end_passes() {
        let check = Check {
            test: CheckTest::Output {
                expected: String::from("yoo     \t\t  \nhello\t \n"),
            },
            name: String::new(),
            args: vec![],
        };
        let output = "yoo\t\nhello\t";

        let event = test(check, output);
        assert!(matches!(event, Event::OutputCheckPassed(..)));
    }
    #[test]
    fn test_new_line_at_the_end_is_passes() {
        let check = Check {
            test: CheckTest::Output {
                expected: String::from("hey there\nhello\n"),
            },
            name: String::new(),
            args: vec![],
        };
        let output = "hey there\nhello";

        let event = test(check, output);
        assert!(matches!(event, Event::OutputCheckPassed(..)));
    }

    #[test]
    fn test_new_word_at_beginning_fails() {
        let check = Check {
            test: CheckTest::Output {
                expected: String::from("hello"),
            },
            name: String::new(),
            args: vec![],
        };
        let output = "world hello";

        let event = test(check, output);
        assert!(matches!(event, Event::OutputCheckFailed(..)));
    }

    #[test]
    fn test_extra_lines_fails() {
        let check = Check {
            test: CheckTest::Output {
                expected: String::from("hello\n\nworld"),
            },
            name: String::new(),
            args: vec![],
        };
        let output = "hello\nworld";

        let event = test(check, output);
        assert!(matches!(event, Event::OutputCheckFailed(..)));
    }
}
