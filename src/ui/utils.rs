/// Utilities like constant, small functions and other UI rendering things for Ratatui
/// shared across multiple pages
use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Stylize},
    text::{Line, Span, Text},
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
pub const LOGO_LEFT_RGB: Color = Color::Rgb(LOGO_LEFT.0, LOGO_LEFT.1, LOGO_LEFT.2);
pub const LOGO_RIGHT_RGB: Color = Color::Rgb(LOGO_RIGHT.0, LOGO_RIGHT.1, LOGO_RIGHT.2);

// FUNCTIONS

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
