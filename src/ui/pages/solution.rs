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

pub fn render_solution(){

}