use crate::models::exo::Exo;
use crate::models::skill::Skill;
use crate::ui::utils::{get_gradient_line, LOGO_LEFT, LOGO_RIGHT};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Text},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
    prelude::Stylize,
};
use std::sync::Arc;

static vertical_areas: Vec<Rect> = vec![];
static horizontal_areas: Vec<Rect> = vec![];


// Define styles using functions instead of constants
fn selected_style() -> Style {
    Style::default().bg(Color::Blue).add_modifier(Modifier::BOLD)
}

fn normal_style() -> Style {
    Style::default().bg(Color::Black)
}

// Main render function for skills and exos selection
pub fn render_skills_selection(
    frame: &mut Frame,
    skills: &Arc<Vec<Skill>>,
    exos: &Arc<Vec<Exo>>,
    skill_index: &usize,
) {
    let (vertical_areas, horizontal_areas) = setup_layout(frame);

    render_header(frame, *vertical_areas[0]);

    render_list(
        frame,
        *horizontal_areas[0],
        "Skills",
        &skills.iter().map(|skill| skill.name.clone()).collect::<Vec<_>>(),
        Some(*skill_index),
    );

    render_list(
        frame,
        *horizontal_areas[1],
        "Exos",
        &exos.iter().map(|exo| exo.name.clone()).collect::<Vec<_>>(),
        None,
    );
}

// Main render function for skills and exos selection with exo highlighting
pub fn render_exos_selection(
    frame: &mut Frame,
    skills: &Arc<Vec<Skill>>,
    exos: &Arc<Vec<Exo>>,
    skill_index: &usize,
    exo_index: &usize,
) {
    setup_layout(frame);

    render_header(frame, *vertical_areas[0]);

    render_list(
        frame,
        horizontal_areas[0],
        "Skills",
        &skills.iter().map(|skill| skill.name.clone()).collect::<Vec<_>>(),
        None,
    );

    render_list(
        frame,
        horizontal_areas[1],
        "Exos",
        &exos.iter().map(|exo| exo.name.clone()).collect::<Vec<_>>(),
        Some(*exo_index),
    );
}

// Renders the header with ASCII art
fn render_header(frame: &mut Frame, area: Rect) {
    let header_text = get_gradient_line("PLX", LOGO_LEFT, LOGO_RIGHT, 3.0);
    let header = Paragraph::new(Text::from(header_text)).left_aligned();
    frame.render_widget(header, area);
}

// Renders a list of items with optional selection highlighting
fn render_list(frame: &mut Frame, area: Rect, title: &str, items: &[String], selected_index: Option<usize>) {
    let list_items: Vec<ListItem> = items
        .iter()
        .map(|item| ListItem::new(Line::from(item.clone())).style(normal_style()))
        .collect();

    let mut state = ListState::default();
    state.select(selected_index);

    let list = List::new(list_items)
        .block(Block::default().borders(Borders::ALL).title(title).style(normal_style()))
        .highlight_style(selected_style())
        .highlight_symbol("> ");

    if selected_index.is_some() {
        frame.render_stateful_widget(list, area, &mut state);
    } else {
        frame.render_widget(list, area);
    }
}

// Sets up the layout for the frame
fn setup_layout(frame: &mut Frame) ->  {
    // Define the vertical layout of the screen
    static vertical_areas: Vec<&Rect> = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(2), Constraint::Min(0)].as_ref())
        .split(frame.area())
        .into_iter()
        .collect::<Vec<_>>();  // Explicitly specify Vec<Rect>

    // Define the horizontal layout for the main content area
    static horizontal_areas: Vec<&Rect> = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(*vertical_areas[1])
        .into_iter()
        .collect::<Vec<_>>();  // Explicitly specify Vec<Rect>

}