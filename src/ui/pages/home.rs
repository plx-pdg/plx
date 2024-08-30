use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

// Function to mix colors and create the gradient result
// factor = float qui indique la pos entre la couleur de first et second.
// on fait les diff entre les r, g, b des 2 couleurs en utilisant le factor.
fn mixed_color(start: (u8, u8, u8), end: (u8, u8, u8), factor: f32) -> (u8, u8, u8) {
    let r = start.0 as f32 + factor * (end.0 as f32 - start.0 as f32);
    let g = start.1 as f32 + factor * (end.1 as f32 - start.1 as f32);
    let b = start.2 as f32 + factor * (end.2 as f32 - start.2 as f32);
    (r as u8, g as u8, b as u8)
}

// From ratatui recipe https://ratatui.rs/recipes/layout/center-a-rect/
pub fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}

pub fn render_home(frame: &mut Frame) {
    let ascii_logo = r#"
████████  ██       ██     ██
██     ██ ██        ██   ██ 
██     ██ ██         ██ ██  
████████  ██          ███   
██        ██         ██ ██  
██        ██        ██   ██ 
██        ████████ ██     ██"#;
    let quick_help_lines = vec![
        "Press 'r' to resume progress",
        "Press 'l' to list all exercices",
        "Press '?' to display help",
    ];

    // Generate a gradient manually on ascii_logo from left to right
    // Ratatui needs RGB => so we need convert hex colors to RGB
    let left_color = (252, 17, 0); // #fc1100 => RGB = 252, 17, 0
    let right_color = (255, 176, 0); // #ffb000 => RGB = 255, 176, 0

    let gradient_width = ascii_logo.lines().map(str::len).max().unwrap_or_default();
    // Split the text in lines and columns to distribute the color of the gradiant
    // Generate a new line per existing line, including a colored char per existing char
    // The color of char is defined via mixed_color() with factor from 0 to 1
    let lines_from_text: Vec<&str> = ascii_logo.lines().collect();
    let mut lines: Vec<Line> = vec![];
    for line in lines_from_text.iter() {
        let mut new_line = Line::default();
        for (j, c) in line.chars().enumerate() {
            //TODO: fix this hacky -30 used to have the gradient approximatively reach right yellow
            let factor = j as f32 / (gradient_width as f32 - 30 as f32) as f32;
            let color = mixed_color(left_color, right_color, factor);
            let style = Style::default().fg(Color::Rgb(color.0, color.1, color.2));
            new_line.push_span(Span::styled(c.to_string(), style));
        }
        lines.push(new_line.clone());
    }
    // Append quick help
    lines.push(Line::from("")); //margin top for quick help
    for l in quick_help_lines {
        lines.push(Line::from(l));
    }

    // The global zone is centered horizontally and vertically !
    // Helped by https://ratatui.rs/recipes/layout/center-a-rect/
    let width = lines.iter().map(|l| l.width()).max().unwrap_or_default();
    let height = lines.len();
    let zone = Paragraph::new(Text::from(lines));
    let centered_area = center(
        frame.area(),
        Constraint::Length(width as u16),
        Constraint::Length(height as u16),
    );

    frame.render_widget(zone, centered_area);
}
