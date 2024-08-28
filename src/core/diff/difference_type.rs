#[derive(Debug, PartialEq, Eq)]
pub(super) enum DifferenceType {
    NoDiff,
    Removed,
    Added,
}
