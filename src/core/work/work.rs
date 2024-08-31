use crate::core::editor::opener::EditorOpener;
use crate::core::watcher::watcher::FileWatcher;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Work {
    EditorOpen(EditorOpener),
    DirectoryWatcher(FileWatcher)
}
