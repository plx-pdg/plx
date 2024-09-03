use crate::core::parser::object_creator;
use crate::models::exo::Exo;
use crate::models::skill::Skill;
use crate::models::ui_state::UiState;
use crate::ui::ui::Ui;
use ratatui::prelude::Text;
use ratatui::style::Stylize;
use ratatui::text::Line;
use ratatui::widgets::Paragraph;
use ratatui::Frame;
use std::sync::mpsc;
// use crate::ui::utils::render_header;

pub fn render_list(frame: &mut Frame, ui_state: &UiState) {
    let mut lines: Vec<Line> = vec![];
    let mut local_exos: Vec<Exo> = vec![];
    let mut local_skills: Vec<Skill> = vec![];
    match ui_state {
        UiState::SkillSelection {
            skill_index,
            skills,
            exos,
        } => {
            lines.push(
                Line::from(format!(
                    "List of skills - selected index {}",
                    skill_index.to_string()
                ))
                .blue()
                .bold(),
            );
            skills
                .iter()
                .for_each(|s| lines.push(Line::from(s.name.clone())));
        }
        UiState::ExoSelection {
            skills,
            skill_index,
            exos,
            exo_index,
        } => {
            lines.push(
                Line::from(format!(
                    "List of skills - selected index {}",
                    skill_index.to_string()
                ))
                .blue()
                .bold(),
            );
            skills
                .iter()
                .for_each(|s| lines.push(Line::from(s.name.clone())));
            lines.push(Line::from("List of exos").green().bold());
            exos.iter()
                .for_each(|e| lines.push(Line::from(e.name.clone())));
        }
        _ => return,
    }

    let zone = Paragraph::new(Text::from(lines)).left_aligned();
    frame.render_widget(zone, frame.area());
}
