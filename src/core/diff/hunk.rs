use super::line::Line;

///Represents a local group of differences
#[derive(Debug, PartialEq, Eq, Clone)]
pub(super) struct Hunk {
    lines: Vec<Line>,
}

impl Hunk {
    pub(super) fn new(lines: Vec<Line>) -> Self {
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
