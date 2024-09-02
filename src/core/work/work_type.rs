#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WorkType {
    EditorOpen,
    Ui,
    DirectoryWatcher,
}
