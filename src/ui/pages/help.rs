use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use strum::IntoEnumIterator;

use crate::models::key::Key;

pub fn render_help(frame: &mut Frame) {
    for k in Key::iter() {
        println!("{}", format!("{:?} {} {}", k, k.describe(), k.alt()));
    }

    // Render the help paragraph in the first chunk
    // frame.render_widget(help_paragraph, chunks[0]);
}
