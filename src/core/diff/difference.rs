use similar::{ChangeTag, TextDiff};

use super::{
    difference_type::DifferenceType, line_chunk::LineChunk, line_difference::LineDifference,
    single_difference::SingleDifference,
};

#[derive(Debug, PartialEq)]
pub struct Difference {
    differences: Vec<SingleDifference>,
}

impl Difference {
    fn new(differences: Vec<SingleDifference>) -> Self {
        Self { differences }
    }
    pub fn to_ansi_colors(&self) -> String {
        let mut result = String::new();
        for (idx, diff) in self.differences.iter().enumerate() {
            if idx > 0 {
                result.push_str(&format!("{:-^1$}", "-", 80));
            }
            result.push_str(&diff.to_ansi_colors());
        }
        result
    }
    pub fn calculate_difference(
        old: &str,
        new: &str,
        lines_between_changes: Option<usize>,
    ) -> Self {
        let diff = TextDiff::from_lines(old, new);

        let mut differences = Vec::new();
        for group in &diff.grouped_ops(lines_between_changes.unwrap_or(3)) {
            let mut lines = Vec::new();
            for op in group {
                for change in diff.iter_inline_changes(op) {
                    let diff_type = match change.tag() {
                        ChangeTag::Delete => DifferenceType::Removed,
                        ChangeTag::Insert => DifferenceType::Added,
                        ChangeTag::Equal => DifferenceType::NoDiff,
                    };

                    let mut line_chunks = Vec::new();
                    for (emphasized, value) in change.iter_strings_lossy() {
                        line_chunks.push(LineChunk::new(value.to_string(), emphasized));
                    }
                    lines.push(LineDifference::new(
                        // change.old_index(),
                        // change.new_index(),
                        line_chunks,
                        change.missing_newline(),
                        diff_type,
                    ))
                }
            }
            differences.push(SingleDifference::new(lines));
        }

        Self::new(differences)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_diff() {
        let old = "Hello\nWorld";
        let new = "Hello\nWorld";
        let diff = Difference::calculate_difference(old, new, None);
        assert!(diff.differences.is_empty());
    }
    #[test]
    fn test_diff() {
        let old = "Hello\nWorld";
        let new = "Hello\nWorld\n";
        let diff = Difference::calculate_difference(old, new, None);
        let expected = Difference {
            differences: vec![SingleDifference::new(vec![
                LineDifference::new(
                    vec![LineChunk::new(String::from("Hello\n"), false)],
                    false,
                    DifferenceType::NoDiff,
                ),
                LineDifference::new(
                    vec![LineChunk::new(String::from("World"), false)],
                    true,
                    DifferenceType::Removed,
                ),
                LineDifference::new(
                    vec![
                        LineChunk::new(String::from("World"), false),
                        LineChunk::new(String::from("\n"), false),
                    ],
                    false,
                    DifferenceType::Added,
                ),
            ])],
        };
        assert_eq!(expected, diff);
    }
    #[test]
    #[ignore = "this test is setup dependent"]
    fn test_ansi_colors() {
        let old = "Hello\nWorld\n";
        let new = "Hello\nWorld Test\n";
        let diff = Difference::calculate_difference(old, new, None);
        let ansi = diff.to_ansi_colors();
        let expected_ansi = r"[2m [0m[2mHello
[0m[31m[1m-[0m[31m[1m[2mWorld[0m[31m[1m[2m
[0m[32m[1m+[0m[32m[1m[2mWorld[0m[32m[1m Test[0m[32m[1m[2m
[0m";
        assert_eq!(expected_ansi, ansi);
    }
}
