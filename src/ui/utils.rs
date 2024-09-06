use crate::models::key::Key;
use crossterm::event::KeyCode;
use ratatui::widgets::Paragraph;
/// Utilities like constant, small functions and other UI rendering things for Ratatui
/// shared across multiple pages
use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Stylize},
    text::{Line, Span, Text},
    Frame,
};

// CONSTANTS

// Branding
pub const ASCII_LOGO: &str = r#"
████████  ██       ██     ██
██     ██ ██        ██   ██ 
██     ██ ██         ██ ██  
████████  ██          ███   
██        ██         ██ ██  
██        ██        ██   ██ 
██        ████████ ██     ██"#;

pub const SLOGAN: &str = "Practice programming exos in a delightful Learning eXperience";
pub const LOGO_LEFT: (u8, u8, u8) = (252, 17, 0); // #fc1100 in RGB => 252, 17, 0
pub const LOGO_RIGHT: (u8, u8, u8) = (255, 176, 0); // #ffb000 in RGB => 255, 176, 0

//pub const LOGO_LEFT_RGB: Color = Color::Rgb(LOGO_LEFT.0, LOGO_LEFT.1, LOGO_LEFT.2);
pub const LOGO_RIGHT_RGB: Color = Color::Rgb(LOGO_RIGHT.0, LOGO_RIGHT.1, LOGO_RIGHT.2);
//pub const EXO_INPROGRESS_COLOR: Color = Color::Rgb(254, 129, 0); // #fe8100
//pub const EXO_DONE_COLOR: Color = Color::Green;

// FUNCTIONS

/// Convert a crossterm::Event::KeyCode to a core::models::key::Key
pub fn ui_key_to_core_key(key: &KeyCode) -> Option<Key> {
    match key {
        KeyCode::Char('q') => Some(Key::Q),
        KeyCode::Char('h') | KeyCode::Left => Some(Key::H),
        KeyCode::Char('j') | KeyCode::Down => Some(Key::J),
        KeyCode::Char('k') | KeyCode::Up => Some(Key::K),
        KeyCode::Char('l') | KeyCode::Right => Some(Key::L),
        KeyCode::Char('r') => Some(Key::R),
        KeyCode::Char('p') => Some(Key::P),
        KeyCode::Char('n') => Some(Key::N),
        KeyCode::Enter => Some(Key::Enter),
        KeyCode::Esc => Some(Key::Esc),
        KeyCode::Char('?') => Some(Key::Interrogation),
        _ => None,
    }
}

/// Get the mixed color in the middle of 2 colors to colorize a specific part in the gradient
pub fn mixed_color(start: (u8, u8, u8), end: (u8, u8, u8), factor: f32) -> (u8, u8, u8) {
    let r = start.0 as f32 + factor * (end.0 as f32 - start.0 as f32);
    let g = start.1 as f32 + factor * (end.1 as f32 - start.1 as f32);
    let b = start.2 as f32 + factor * (end.2 as f32 - start.2 as f32);
    (r as u8, g as u8, b as u8)
}

/// Function to get a Line from a given string, applying a gradient from left to right
/// the width value is the width of the given text or smaller one if gradient doesn't work
/// The Line combine one Span per char and each char is colored differently
/// The color of char is defined via mixed_color() with factor from 0 to 1
pub fn get_gradient_line(
    line: &str,
    left_color: (u8, u8, u8),
    right_color: (u8, u8, u8),
    width: f32,
) -> Line {
    let mut new_line = Line::default();
    for (j, c) in line.chars().enumerate() {
        let factor = j as f32 / width;
        let color = mixed_color(left_color, right_color, factor);
        new_line.push_span(Span::from(c.to_string()).fg(Color::Rgb(color.0, color.1, color.2)));
    }
    new_line
}

// From ratatui recipe https://ratatui.rs/recipes/layout/center-a-rect/
/// Center a given rectangle with given horizontal and vertical constraints
pub fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}

/// Renders the header with ASCII art
pub(crate) fn render_header(frame: &mut Frame, area: Rect) {
    let header_text = get_gradient_line("PLX", LOGO_LEFT, LOGO_RIGHT, 3.0);
    let header = Paragraph::new(Text::from(header_text)).left_aligned();
    frame.render_widget(header, area);
}

// From ratatui example app: https://ratatui.rs/examples/apps/popup/
// We might change to a custom Popup Widget if we need more widgets, see more on https://ratatui.rs/recipes/render/overwrite-regions/
/// Helper function to create a centered rect using up certain percentage of the available rect `r`
pub fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}
