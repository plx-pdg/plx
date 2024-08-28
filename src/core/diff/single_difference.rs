use super::difference_operation::DifferenceOperation;

#[derive(Debug, PartialEq, Eq)]
pub(super) struct SingleDifference {
    operations: Vec<DifferenceOperation>,
}

impl SingleDifference {
    pub(super) fn new(operations: Vec<DifferenceOperation>) -> Self {
        Self { operations }
    }
    pub(super) fn to_ansi_colors(&self) -> String {
        self.operations
            .iter()
            .map(|op| op.to_ansi_colors())
            .collect::<Vec<String>>()
            .join("")
    }
}
