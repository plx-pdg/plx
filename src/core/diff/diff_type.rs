#[derive(Debug, PartialEq, Eq)]
pub(super) enum DiffType {
    NoDiff,
    Removed,
    Added,
}
