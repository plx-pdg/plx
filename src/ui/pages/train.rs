use std::sync::Arc;

use crate::models::exo::Exo;
use ratatui::{
    style::Stylize,
    text::{Line, Text},
    widgets::{Paragraph, Wrap},
    Frame,
};

// Show the "Compiling" message
pub fn render_compilation(frame: &mut Frame, exo: &Arc<Exo>) {
    let mut bottom: Vec<Line> = vec![];
    bottom.push(Line::from("Compiling...").bold().yellow());
    render_train(frame, exo, bottom);
}

// Show the compilation errors, basic support of offset to scroll in long output
pub fn render_compilation_error(
    frame: &mut Frame,
    exo: &Arc<Exo>,
    scroll_offset: &usize,
    error: &String,
) {
    let mut bottom: Vec<Line> = vec![];
    bottom.push(Line::from("Compilation errors").bold().red());
    let output: String = error.lines().skip(*scroll_offset).collect();
    bottom.push(Line::from(output));
    // bottom.push(Line::from(error.lines().skip(scroll_offset).collect())); //TODO fix this

    for (i, check) in exo.checks.clone().iter().enumerate() {
        bottom.push(Line::from(format!("{}. {}", i + 1, check.name)));
    }

    render_train(frame, exo, bottom);
}

// The common top part with exo name and instruction
fn render_common_top(lines: &mut Vec<Line>, exo: &Arc<Exo>) {
    lines.push(Line::from(exo.name.clone()).cyan().bold());
    if let Some(instr) = exo.instruction.clone() {
        lines.push(Line::from(instr).magenta());
    }
}
// Render final page with the common top and specific bottom
fn render_train(frame: &mut Frame, exo: &Arc<Exo>, bottom: Vec<Line>) {
    let mut lines: Vec<Line> = vec![];
    render_common_top(&mut lines, exo);
    lines.push(Line::default());
    bottom.iter().for_each(|l| lines.push(l.clone()));
    let mut zone = Paragraph::new(Text::from(lines)).wrap(Wrap { trim: true });
    frame.render_widget(zone, frame.area());
}
