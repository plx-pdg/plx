use crate::core::diff::diff::Diff;

use super::key::Key;

/// Represents every possible app event
#[derive(Debug, PartialEq, Eq)]
pub enum Event {
    KeyPressed(Key),
    EditorOpened,
    CouldNotOpenEditor,
    CompilationStart,
    CompilationEnd(bool),
    CompilationOutputLine(String),
    FileSaved,
    OutputCheckPassed(usize),
    OutputCheckFailed(usize, Diff),
    RunStart(usize),
    RunEnd(usize),
    RunOutputLine(usize, String),
    RunFail(usize, String),
}
