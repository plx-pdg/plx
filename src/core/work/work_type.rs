#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WorkType {
    EditorOpen,
    Compilation,
    Ui,
    DirectoryWatcher,
}
