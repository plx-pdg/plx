use ratatui::{
    layout::Constraint,
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Row, Table},
    Frame,
};
use strum::IntoEnumIterator;

use crate::models::key::Key;

pub fn render_help(frame: &mut Frame, _scroll_offset: usize) {
    // USING TABLE
    let mut rows = Vec::new();
    let mut max_shortcut_text_length: u16 = 0;
    for k in Key::iter() {
        let description = Line::from(format!("{}", k.describe()))
            .style(Style::default().fg(Color::White))
            .dim();

        let mut shortcut_text = Line::default();
        let shortcut_name = &k.name();
        shortcut_text.push_span(Span::from(shortcut_name.clone()).green());
        let mut length_count: u16 = shortcut_name.len() as u16;
        let alt = k.alt().to_string();
        if !alt.is_empty() {
            shortcut_text.push_span(Span::from(", ").dim());
            shortcut_text.push_span(Span::from(alt).green());
            length_count += k.alt().len() as u16 + 2;
        }

        if length_count > max_shortcut_text_length {
            max_shortcut_text_length = length_count;
        }

        let row = Row::new(vec![Line::from(shortcut_text), description]);
        rows.push(row);
    }

    //TODO: skip first rows to respect given offset to be able to scroll down

    // Create a table with 2 columns: | shortcut, alt | description |
    let table = Table::new(rows, vec![Constraint::Percentage(100)])
        .block(Block::default().title("Help of shortcuts"))
        .widths(&[
            Constraint::Min(max_shortcut_text_length),
            Constraint::Percentage(90),
        ]);

    frame.render_widget(table, frame.area());
}
