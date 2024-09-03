use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, List, ListState, Table},
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

    // Collect all items into a single list
    let mut items = Vec::new();
    for k in Key::iter() {
        let key = Text::from(format!("{:?}", k)).style(Style::default().fg(Color::Green));

        let description =
            Text::from(format!("{:?}", k.describe())).style(Style::default().fg(Color::Blue));

        let alt = Text::from(format!("{:?}", k.alt()));

        // Combine all 3 items into a single line
        // if alt is different from empty string then add it to the line

        if alt != Text::from("") {
            let combined_text = format!("{} {} (alt: {})", key, description, alt);
            items.push(combined_text);
        } else {
            let combined_text = format!("{} {}", key, description);
            items.push(combined_text);
        }
    }

    // Create a single list with all items
    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Key Descriptions"),
    );

    // Render the list inside the inner layout
    frame.render_widget(list, layout[0]);
}
