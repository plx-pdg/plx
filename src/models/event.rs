use crate::core::diff::diff::Diff;

use super::key::Key;

#[derive(Debug, PartialEq, Eq)]
pub enum Event {
    KeyPressed(Key),
    EditorOpened,
    CouldNotOpenEditor,
    OutputCheckPassed(usize),
    OutputCheckFailed(usize, Diff),
}
