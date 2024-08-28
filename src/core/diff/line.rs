use console::Style;

use super::{diff_type::DiffType, line_chunk::LineChunk};

#[derive(Debug, PartialEq)]
pub(super) struct Line {
    // old_line_index: Option<usize>, // Where this line was before
    // new_line_index: Option<usize>, // Where this line is now
    line_chunks: Vec<LineChunk>,
    missing_new_line: bool,
    difference_type: DiffType,
}
impl Line {
    pub(super) fn new(
        // old_line_index: Option<usize>,
        // new_line_index: Option<usize>,
        line_chunks: Vec<LineChunk>,
        missing_new_line: bool,
        difference_type: DiffType,
    ) -> Self {
        Self {
            // old_line_index,
            // new_line_index,
            line_chunks,
            missing_new_line,
            difference_type,
        }
    }
    pub(super) fn to_ansi_colors(&self) -> String {
        let mut result = String::new();
        let (sign, s) = match self.difference_type {
            DiffType::Removed => ("-", Style::new().red().bold()),
            DiffType::Added => ("+", Style::new().green().bold()),
            DiffType::NoDiff => (" ", Style::new().dim()),
        };

        result.push_str(&format!("{}", s.apply_to(sign)));
        for chunk in &self.line_chunks {
            result.push_str(&chunk.to_ansi_colors(&s));
        }
        if self.missing_new_line {
            result.push('\n');
        }
        result
    }
}
