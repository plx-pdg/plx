use std::sync::mpsc;
use ratatui::Frame;
use ratatui::prelude::Text;
use ratatui::text::Line;
use ratatui::widgets::Paragraph;
use crate::core::parser::object_creator;
use crate::models::ui_state::UiState;
use crate::ui::ui::Ui;
use crate::ui::utils::render_header;

pub fn render_list(frame: &mut Frame, ui_state: &UiState) {
    let mut lines: Vec<Line> = vec![];

    let zone = Paragraph::new(Text::from(lines)).left_aligned();
    frame.render_widget(zone, Default::default());
}
