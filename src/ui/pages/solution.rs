use crate::models::exo::Exo;
use crate::ui::utils::{get_gradient_line, render_header, LOGO_LEFT, LOGO_RIGHT};
use ratatui::prelude::Span;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Text},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};
use std::path::PathBuf;
use std::sync::Arc;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style as SyntectStyle, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

pub fn render_solution(
    frame: &mut Frame,
    exo: &Arc<Exo>,
    solution: &String,
    solution_path: &PathBuf,
    solution_idx: &usize,
) {
    let binding = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2), Constraint::Min(0)].as_ref())
        .split(frame.area());
    let vertical = binding.into_iter().collect::<Vec<_>>(); // Explicitly specify Vec<Rect>

    render_header(frame, vertical[0].clone());

    let syntax_set = SyntaxSet::load_defaults_newlines();
    let theme_set = ThemeSet::load_defaults();
    let syntax = syntax_set.find_syntax_by_extension("c").unwrap();
    let theme = &theme_set.themes["base16-eighties.dark"];
    let mut highlighter = HighlightLines::new(syntax, theme);
    let mut highlighted_lines: Vec<Line> = Vec::new();

    // Highlight each line of the code
    for line in LinesWithEndings::from(&solution) {
        // Highlight the line
        let ranges: Vec<(SyntectStyle, &str)> =
            highlighter.highlight_line(line, &syntax_set).unwrap();

        // Convert each segment of the highlighted line into a Span
        let spans: Vec<Span> = ranges
            .iter()
            .map(|(style, text)| {
                // Map syntect style to ratatui style
                let fg = style.foreground;
                let tui_style = Style::default().fg(Color::Rgb(fg.r, fg.g, fg.b));
                Span::styled(*text, tui_style)
            })
            .collect();

        // Add the highlighted line as a Line object
        highlighted_lines.push(Line::from(spans));
    }

    // Create a Paragraph widget with the highlighted lines
    let paragraph = Paragraph::new(highlighted_lines)
        .block(Block::default().title("Solution").borders(Borders::ALL))
        .wrap(ratatui::widgets::Wrap { trim: false });

    frame.render_widget(paragraph, vertical[1].clone());
}
