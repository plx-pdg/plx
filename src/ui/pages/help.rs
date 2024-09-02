use ratatui::{layout::{Constraint, Direction, Layout}, style::{Color, Style, Stylize}, text::{Line, Span, Text}, widgets::{Block, Borders, Paragraph}, Frame};

pub fn render_help(frame: &mut Frame) {
    //TODO

    let quick_help_lines = vec![    
        "Type r to resume progress",
        "Type l to list all exos",
        "Type ? to display help",
        "Type q to quit",
    ];

    let mut lines: Vec<Line> = vec![];
    lines.push(Line::default()); //margin top
    lines.push(Line::from("Quick help").bold()); //margin top for quick help
    for l in quick_help_lines {
        lines.push(Line::from(l).dim());
    }

    let help_text = quick_help_lines
        .iter()
        .map(|&line| Span::from(Span::raw(line)))
        .collect::<Vec<_>>();

    let help_paragraph = Paragraph::new(Text::from(help_text))
        .block(Block::default().borders(Borders::ALL).title("Quick Help"))
        .style(Style::default().fg(Color::White).bg(Color::Black));

    // Set up the layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(frame.area());

    // Render the help paragraph in the first chunk
    frame.render_widget(help_paragraph, chunks[0]);
}