use std::sync::{atomic::AtomicBool, mpsc::Sender, Arc};

use crate::{
    core::{
        diff::diff::Diff,
        work::{work::Work, work_type::WorkType},
    },
    models::event::Event,
};

#[derive(Debug)]
pub enum OutputCheckerCreationError {
    InvalidCheck,
}
pub struct OutputChecker<'a> {
    id: usize,
    expected: &'a str,
    program_output: &'a str,
}

impl<'a> OutputChecker<'a> {
    pub fn new(id: usize, program_output: &'a str, expected: &'a str) -> Self {
        Self {
            id,
            expected,
            program_output,
        }
    }
}
impl Work for OutputChecker<'_> {
    fn run(&self, tx: Sender<Event>, _stop: Arc<AtomicBool>) -> bool {
        let diff = Diff::calculate_difference(&self.program_output, self.expected, None);

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
    fn test(expected: &str, output: &str) -> Event {
        let (tx, rx) = channel();

        let checker = OutputChecker::new(0, expected, output);
        checker.run(tx, Arc::new(AtomicBool::new(false)));

        rx.recv().unwrap()
    }
    #[test]
    fn test_id() {
        let output = "hello";
        let (tx, rx) = channel();

        let checker = OutputChecker::new(0, "hello", output);
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
        let output = "hello";
        let event = test("hello", output);
        assert!(matches!(event, Event::OutputCheckPassed(_)));
    }

    #[test]
    fn test_extra_word_at_the_end_fails() {
        let output = "hello world";
        let event = test("hello", output);
        assert!(matches!(event, Event::OutputCheckFailed(..)));
    }

    #[test]
    fn test_extra_whitespace_at_the_end_passes() {
        let output = "hello ";
        let event = test("hello", output);
        assert!(matches!(event, Event::OutputCheckPassed(..)));
    }

    #[test]
    fn test_extra_whitespace_beggining_fails() {
        let output = "hey\n hello";
        let event = test("hey\nhello", output);
        assert!(matches!(event, Event::OutputCheckFailed(..)));
    }

    #[test]
    fn test_tab_at_the_end_passes() {
        let output = "yoo\t\nhello\t";
        let event = test("yoo\nhello", output);
        assert!(matches!(event, Event::OutputCheckPassed(..)));
    }

    #[test]
    fn test_mix_of_whitespaces_at_end_passes() {
        let output = "yoo\t\nhello\t";

        let event = test("yoo     \t\t  \nhello\t \n", output);
        assert!(matches!(event, Event::OutputCheckPassed(..)));
    }
    #[test]
    fn test_new_line_at_the_end_is_passes() {
        let output = "hey there\nhello";
        let event = test("hey there\nhello\n", output);
        assert!(matches!(event, Event::OutputCheckPassed(..)));
    }

    #[test]
    fn test_new_word_at_beginning_fails() {
        let output = "world hello";

        let event = test("hello", output);
        assert!(matches!(event, Event::OutputCheckFailed(..)));
    }

    #[test]
    fn test_extra_lines_fails() {
        let output = "hello\nworld";

        let event = test("hello\n\nworld", output);
        assert!(matches!(event, Event::OutputCheckFailed(..)));
    }
}
