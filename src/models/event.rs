use crate::core::diff::difference::Difference;

use super::key::Key;

#[derive(Debug, PartialEq, Eq)]
pub enum Event {
    KeyPressed(Key),
    EditorOpened,
    CouldNotOpenEditor,
    OutputCheckPassed(usize),
    OutputCheckFailed(usize, Difference),
}
