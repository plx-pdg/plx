use std::sync::Arc;

use crate::models::{check::CheckTest, check_state::CheckState, exo::Exo};
use ratatui::{
    style::{Color, Stylize},
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

    render_train(frame, exo, bottom);
}

// Show the check results
pub fn render_check_results(
    frame: &mut Frame,
    exo: &Arc<Exo>,
    scroll_offset: &usize,
    checks: &Vec<CheckState>,
) {
    let mut bottom: Vec<Line> = vec![];
    let passed_checks_count = checks.iter().filter(|c| c.passed).count();
    bottom.push(
        Line::from(format!(
            "Check results - {}/{} passed",
            passed_checks_count,
            checks.len()
        ))
        .bold()
        .green(),
    );
    // bottom.push(Line::from(error.lines().skip(scroll_offset).collect())); //TODO fix this

    for (i, check_state) in checks.clone().iter().enumerate() {
        let color = if check_state.passed {
            Color::Green
        } else {
            Color::Red
        };
        bottom.push(Line::from(
            format!("{}. {}", i + 1, check_state.check.name)
                .fg(color)
                .bold(),
        ));
        if check_state.passed {
            continue;
        }
        if !check_state.check.args.is_empty() {
            bottom.push(Line::from(format!("Args: {:?}", check_state.check.args)).magenta());
        }
        match check_state.check.test.clone() {
            CheckTest::Output { expected } => {
                bottom.push(Line::from("Expected:"));
                bottom.push(Line::from(format!("{}", expected)).blue());
            }
        }
        bottom.push(Line::default());
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
    let zone = Paragraph::new(Text::from(lines)).wrap(Wrap { trim: true });
    frame.render_widget(zone, frame.area());
}
