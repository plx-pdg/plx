use super::line_difference::LineDifference;

#[derive(Debug, PartialEq)]
pub(super) struct DifferenceOperation {
    lines: Vec<LineDifference>,
}
impl DifferenceOperation {
    pub(super) fn new(lines: Vec<LineDifference>) -> Self {
        Self { lines }
    }
    pub(super) fn to_ansi_colors(&self) -> String {
        self.lines
            .iter()
            .map(|line| line.to_ansi_colors())
            .collect::<Vec<String>>()
            .join("")
    }
}
