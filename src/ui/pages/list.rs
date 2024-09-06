use crate::models::exo::Exo;
use crate::models::exo_state::ExoState;
use crate::models::skill::Skill;
use crate::ui::pages::train::render_exo;
use crate::ui::utils::{
    get_gradient_line, popup_area, render_header, EXO_DONE_COLOR, EXO_INPROGRESS_COLOR, LOGO_LEFT,
    LOGO_RIGHT,
};
use ratatui::style::Stylize;
use ratatui::widgets::{Clear, Wrap};
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

/// Render the 2 lists with a popup preview showing content from train::render_exo()
pub fn render_preview(
    frame: &mut Frame,
    skills: &Arc<Vec<Skill>>,
    exos: &Arc<Vec<Exo>>,
    skill_index: &usize,
    exo_index: usize,
) {
    render_lists(frame, skills, exos, skill_index, Some(exo_index), false);
    let mut lines: Vec<Line> = Vec::new();
    render_exo(&mut lines, &Arc::new(exos[exo_index].clone()), true);

    let area = popup_area(frame.area(), 80, 60);
    frame.render_widget(Clear, area); //this clears out the background
    frame.render_widget(
        Paragraph::new(Text::from(lines))
            .block(Block::bordered().title("Exo preview"))
            .wrap(Wrap { trim: true }),
        area,
    );
}

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
            .map(|skill| Line::from(skill.name.clone()))
            .collect::<Vec<Line>>(),
        Some(*skill_index),
        is_skill_selection,
    );

    render_list(
        frame,
        exos_area,
        "Exos",
        &exos
            .iter()
            .map(|exo| {
                Line::from(exo.name.clone()).fg(match exo.state {
                    ExoState::Todo => Color::default(),
                    ExoState::InProgress => EXO_INPROGRESS_COLOR,
                    ExoState::Done => EXO_DONE_COLOR,
                })
            })
            .collect::<Vec<Line>>(),
        exo_index,
        !is_skill_selection,
    );
}

/// Renders a list of items with optional selection highlighting
fn render_list(
    frame: &mut Frame,
    area: Rect,
    title: &str,
    items: &Vec<Line>,
    selected_index: Option<usize>,
    active_list: bool, // change the border style when the list is active
) {
    let list_items: Vec<ListItem> = items
        .iter()
        .map(|item| ListItem::new(item.clone()).style(DEFAULT_STYLE))
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
