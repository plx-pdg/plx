use crate::core::diff::diff::Diff;

use super::key::Key;

#[derive(Debug, PartialEq, Eq)]
pub enum Event {
    KeyPressed(Key),
    EditorOpened,
    CouldNotOpenEditor,
    CompilationStart,
    CompilationEnd(bool),
    CompilationOutputLine(String),
    FileSaved,
    ProcessCreationFailed,
    ProcessOutputLine(usize, String),
    OutputCheckPassed(usize),
    OutputCheckFailed(usize, Diff),
}
