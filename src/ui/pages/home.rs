use crate::ui::utils::{
    center, get_gradient_line, ASCII_LOGO, LOGO_LEFT, LOGO_RIGHT, LOGO_RIGHT_RGB, SLOGAN,
};
use ratatui::{
    layout::Constraint,
    style::Stylize,
    text::{Line, Span, Text},
    widgets::Paragraph,
    Frame,
};

pub fn render_home(frame: &mut Frame) {
    // Generate a gradient manually on ASCII_LOGO from left to right
    // Split the text in lines and columns to distribute the color of the gradient
    let gradient_width = ASCII_LOGO.lines().map(str::len).max().unwrap_or_default();
    let mut lines: Vec<Line> = vec![];
    for line in ASCII_LOGO.lines() {
        lines.push(get_gradient_line(
            line,
            LOGO_LEFT,
            LOGO_RIGHT,
            //TODO: fix this hacky -30 to have the gradient approximatively reach right yellow
            gradient_width as f32 - 30 as f32,
        ));
    }

    // Append slogan + version line
    let mut slogan_line = get_gradient_line(SLOGAN, LOGO_LEFT, LOGO_RIGHT, SLOGAN.len() as f32);
    lines.push(Line::default()); //margin top
    slogan_line.push_span(Span::from(" - ").dim());
    let version = Span::from(format!("v{}", env!("CARGO_PKG_VERSION"))).fg(LOGO_RIGHT_RGB);
    slogan_line.push_span(version);
    lines.push(slogan_line);

    // Append website and repository
    lines.push(Line::from("Repository: https://github.com/plx-pdg/plx").dim());
    lines.push(Line::from("Website: https://plx.rs").dim());

    // Append quick help
    let quick_help_lines = vec![
        "Type r to resume progress",
        "Type l to list all exos",
        "Type ? to display help",
        "Type q to quit",
    ];
    lines.push(Line::default()); //margin top
    lines.push(Line::from("Quick help").bold()); //margin top for quick help
    for l in quick_help_lines {
        lines.push(Line::from(l).dim());
    }

    // The global zone is centered horizontally and vertically !
    // Helped by https://ratatui.rs/recipes/layout/center-a-rect/
    let width = lines.iter().map(|l| l.width()).max().unwrap_or_default();
    let height = lines.len();
    let zone = Paragraph::new(Text::from(lines)).centered();
    let centered_area = center(
        frame.area(),
        Constraint::Length(width as u16),
        Constraint::Length(height as u16),
    );

    frame.render_widget(zone, centered_area);
}
