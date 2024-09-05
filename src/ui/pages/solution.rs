use crate::ui::utils::{get_gradient_line, render_header, LOGO_LEFT, LOGO_RIGHT};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Text},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};
use std::sync::Arc;
use ratatui::prelude::Span;

pub fn render_solution(frame: &mut Frame, code: String) {

    let binding = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2), Constraint::Min(0)].as_ref())
        .split(frame.area());
    let vertical = binding.into_iter().collect::<Vec<_>>(); // Explicitly specify Vec<Rect>

    render_header(frame, vertical[0].clone());

    let code = String::from("#include <stdio.h>

int main(int argc, char *argv[]) {
  if (argc < 3)
    printf(\"Error: missing argument firstname and legs number\");
  else
    printf(\"The dog is %s and has %s legs\\n\", argv[1], argv[2]);
}");

    let lines: Vec<Line> = code
        .lines()
        .map(|line| Line::from(vec![Span::raw(line)]))
        .collect();

    let paragraph = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL))
        .wrap(ratatui::widgets::Wrap { trim: false });


    frame.render_widget(paragraph, vertical[1].clone());
}
