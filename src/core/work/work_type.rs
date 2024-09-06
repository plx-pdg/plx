/// Defines all possible work types
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WorkType {
    EditorOpen,
    Compilation,
    Ui,
    DirectoryWatcher,
    OutputChecker,
    Launcher,
    Checker,
    Watcher,
}
