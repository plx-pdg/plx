use crate::models::exo::Exo;
use crate::models::skill::Skill;
use crate::ui::utils::{get_gradient_line, LOGO_LEFT, LOGO_RIGHT};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Text},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};
use std::sync::Arc;

/// Define the default, selected border and selected item styles
const DEFAULT_STYLE: Style = Style::new();
const SELECTED_BORDER: Style = Style::new().fg(Color::Blue);
const SELECTED_STYLE: Style = Style::new().fg(Color::Blue).add_modifier(Modifier::BOLD);

/// Main render function for skills and exos, can be used for SkillSelection and ExoSelection !
pub fn render_lists(
    frame: &mut Frame,
    skills: &Arc<Vec<Skill>>,
    exos: &Arc<Vec<Exo>>,
    skill_index: &usize,
    exo_index: Option<usize>,
    is_skill_selection: bool,
) {
    let (skills_area, exos_area) = setup_layout(frame);

    render_list(
        frame,
        skills_area,
        "Skills",
        &skills
            .iter()
            .map(|skill| skill.name.clone())
            .collect::<Vec<_>>(),
        Some(*skill_index),
        is_skill_selection,
    );

    render_list(
        frame,
        exos_area,
        "Exos",
        &exos.iter().map(|exo| exo.name.clone()).collect::<Vec<_>>(),
        exo_index,
        !is_skill_selection,
    );
}

/// Renders the header with ASCII art
fn render_header(frame: &mut Frame, area: Rect) {
    let header_text = get_gradient_line("PLX", LOGO_LEFT, LOGO_RIGHT, 3.0);
    let header = Paragraph::new(Text::from(header_text)).left_aligned();
    frame.render_widget(header, area);
}

/// Renders a list of items with optional selection highlighting
fn render_list(
    frame: &mut Frame,
    area: Rect,
    title: &str,
    items: &[String],
    selected_index: Option<usize>,
    active_list: bool, // change the border style when the list is active
) {
    let list_items: Vec<ListItem> = items
        .iter()
        .map(|item| ListItem::new(Line::from(item.clone())).style(DEFAULT_STYLE))
        .collect();

    let mut state = ListState::default();
    state.select(selected_index);

    let list = List::new(list_items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(if active_list {
                    SELECTED_BORDER
                } else {
                    DEFAULT_STYLE
                })
                .title(title)
                .style(DEFAULT_STYLE),
        )
        .highlight_style(SELECTED_STYLE)
        .highlight_symbol("> ");

    if selected_index.is_some() {
        frame.render_stateful_widget(list, area, &mut state);
    } else {
        frame.render_widget(list, area);
    }
}

/// Set up the layout for the frame, render the header, return 2 areas for the 2 list
fn setup_layout(frame: &mut Frame) -> (Rect, Rect) {
    // Define the vertical layout of the screen
    let binding = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2), Constraint::Min(0)].as_ref())
        .split(frame.area());
    let vertical = binding.into_iter().collect::<Vec<_>>(); // Explicitly specify Vec<Rect>

    render_header(frame, vertical[0].clone());

    // Define the horizontal layout for the main content area
    let binding = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)].as_ref())
        .split(*vertical[1]);
    let horizontal = binding.into_iter().collect::<Vec<_>>(); // Explicitly specify Vec<Rect>

    (horizontal[0].clone(), horizontal[1].clone())
}

