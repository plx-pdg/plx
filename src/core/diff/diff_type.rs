#[derive(Debug, PartialEq, Eq, Clone)]
pub(super) enum DiffType {
    NoDiff,
    Removed,
    Added,
}
