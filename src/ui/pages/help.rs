use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::Text,
    widgets::{Block, Borders, Row, Table},
    Frame,
};
use strum::IntoEnumIterator;

use crate::models::key::Key;

pub fn render_help(frame: &mut Frame) {
    // Create a layout with a full screen display
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(frame.area());

    // Create a block with borders
    let block = Block::default().borders(Borders::NONE).title("Help");

    // Render the block with borders
    frame.render_widget(block, layout[0]);

    // USING TABLE
    let mut rows = Vec::new();
    for k in Key::iter() {
        let key = Text::from(format!("{:?}", k))
            .style(Style::default().fg(Color::Green))
            .alignment(ratatui::layout::Alignment::Left);

        let description = Text::from(format!("{}", k.describe()))
            .style(Style::default().fg(Color::White))
            .centered();

        let alt = Text::from(format!("{}", k.alt()))
            .style(Style::default().fg(Color::Blue))
            .alignment(ratatui::layout::Alignment::Right);

        let row = Row::new(vec![key, description, alt]);
        rows.push(row);

    }

    // Create a table with 3 columns
    let table = Table::new(rows, vec![Constraint::Percentage(100)])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Key Descriptions"),
        )
        .widths(&[
            Constraint::Percentage(33),
            Constraint::Percentage(34),
            Constraint::Percentage(33),
        ]);

    frame.render_widget(table, layout[0]);
}
