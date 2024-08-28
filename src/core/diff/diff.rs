use similar::{ChangeTag, TextDiff};

use super::{diff_type::DiffType, hunk::Hunk, line::Line, line_chunk::LineChunk};

#[derive(Debug, PartialEq)]
pub struct Diff {
    differences: Vec<Hunk>,
}

impl Diff {
    fn new(differences: Vec<Hunk>) -> Self {
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
    // Based on similar sample https://github.com/mitsuhiko/similar/blob/844769ae19f7451c5a5be3505d8865100dd300a0/examples/terminal-inline.rs
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
                        ChangeTag::Delete => DiffType::Removed,
                        ChangeTag::Insert => DiffType::Added,
                        ChangeTag::Equal => DiffType::NoDiff,
                    };

                    let mut line_chunks = Vec::new();
                    for (emphasized, value) in change.iter_strings_lossy() {
                        line_chunks.push(LineChunk::new(value.to_string(), emphasized));
                    }
                    lines.push(Line::new(
                        // change.old_index(),
                        // change.new_index(),
                        line_chunks,
                        change.missing_newline(),
                        diff_type,
                    ))
                }
            }
            differences.push(Hunk::new(lines));
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
        let diff = Diff::calculate_difference(old, new, None);
        println!("{}", diff.to_ansi_colors());
        assert!(diff.differences.is_empty());
    }
    #[test]
    fn test_diff() {
        let old = "Hello\nWorld";
        let new = "Hello\nWorld\n";
        let diff = Diff::calculate_difference(old, new, None);
        let expected = Diff {
            differences: vec![Hunk::new(vec![
                Line::new(
                    vec![LineChunk::new(String::from("Hello\n"), false)],
                    false,
                    DiffType::NoDiff,
                ),
                Line::new(
                    vec![LineChunk::new(String::from("World"), false)],
                    true,
                    DiffType::Removed,
                ),
                Line::new(
                    vec![
                        LineChunk::new(String::from("World"), false),
                        LineChunk::new(String::from("\n"), false),
                    ],
                    false,
                    DiffType::Added,
                ),
            ])],
        };
        println!("{}", diff.to_ansi_colors());
        assert_eq!(expected, diff);
    }
    #[test]
    fn test_diff_new_word() {
        let old = "Hello World";
        let new = "Hello World there";
        let diff = Diff::calculate_difference(old, new, None);
        let expected = Diff {
            differences: vec![Hunk::new(vec![
                Line::new(
                    vec![LineChunk::new(String::from("Hello World"), false)],
                    true,
                    DiffType::Removed,
                ),
                Line::new(
                    vec![
                        LineChunk::new(String::from("Hello World"), false),
                        LineChunk::new(String::from(" there"), true),
                    ],
                    true,
                    DiffType::Added,
                ),
            ])],
        };
        println!("{}", diff.to_ansi_colors());
        assert_eq!(expected, diff);
    }
    #[test]
    fn test_ansi_colors() {
        console::set_colors_enabled(true);
        let old = "Hello\nWorld\n";
        let new = "Hello\nWorld Test\n";
        let diff = Diff::calculate_difference(old, new, None);
        let ansi = diff.to_ansi_colors();
        let expected_ansi = r"[2m [0m[2mHello
[0m[31m[1m-[0m[31m[1m[2mWorld[0m[31m[1m[2m
[0m[32m[1m+[0m[32m[1m[2mWorld[0m[32m[1m Test[0m[32m[1m[2m
[0m";
        println!("{}", diff.to_ansi_colors());
        assert_eq!(expected_ansi, ansi);
    }
}
