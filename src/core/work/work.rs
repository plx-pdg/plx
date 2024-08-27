use crate::core::editor::opener::EditorOpener;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Work {
    EditorOpen(EditorOpener),
}
