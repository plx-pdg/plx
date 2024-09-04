/// Train page rendering functions
/// Each UiState related to this page has its own function and reuse render_train()
/// that generate the render_common_top() to show exo metadata that should be always visible
use std::sync::Arc;

use crate::models::{
    check::CheckTest,
    check_state::{CheckState, CheckStatus},
    exo::Exo,
};
use ratatui::{
    style::{Color, Stylize},
    text::{Line, Span, Text},
    widgets::{Paragraph, Wrap},
    Frame,
};

// Show the "Compiling" message without the checks
pub fn render_compilation(frame: &mut Frame, exo: &Arc<Exo>) {
    let mut bottom: Vec<Line> = vec![];
    bottom.push(Line::from("Compiling...").bold().yellow());
    render_train(frame, exo, bottom);
}

// Show the compilation errors, basic support of scroll_offset
pub fn render_compilation_error(
    frame: &mut Frame,
    exo: &Arc<Exo>,
    scroll_offset: &usize,
    error: &String,
) {
    // Compilation title with offset if > 0
    let mut bottom: Vec<Line> = vec![];
    let mut compilation_title_with_offset =
        Line::from(Span::from("Compilation errors").bold().red());
    //TODO: is it useful and intuitive this offset show format ?
    if *scroll_offset > 0 as usize {
        compilation_title_with_offset.push_span(Span::from(format!(" (>>{})", scroll_offset)));
    }
    bottom.push(compilation_title_with_offset);

    // Show lines of compilation output, each is its own Line
    error
        .lines()
        .skip(*scroll_offset)
        .for_each(|l| bottom.push(Line::from(l.to_string())));

    render_train(frame, exo, bottom);
}

// Show the check results
pub fn render_check_results(
    frame: &mut Frame,
    exo: &Arc<Exo>,
    _scroll_offset: &usize, //TODO: support scroll_offset
    checks: &Vec<CheckState>,
) {
    // Show the Check results title
    let mut bottom: Vec<Line> = vec![];
    let passed_checks_count = checks
        .iter()
        .filter(|c| c.status == CheckStatus::Passed)
        .count();
    bottom.push(
        Line::from(format!(
            "Check results - {}/{} passed",
            passed_checks_count,
            checks.len()
        ))
        .bold()
        .green(),
    );

    // Show each check name + details
    for (i, check_state) in checks.iter().enumerate() {
        let color = if check_state.status == CheckStatus::Passed {
            Color::Green
        } else {
            Color::Red
        };
        bottom.push(Line::from(
            format!("{}. {}", i + 1, check_state.check.name)
                .fg(color)
                .bold(),
        ));
        if check_state.status == CheckStatus::Passed {
            continue;
        }
        if !check_state.check.args.is_empty() {
            bottom.push(Line::from(format!("Args: {:?}", check_state.check.args)).magenta());
        }
        match check_state.check.test.clone() {
            CheckTest::Output { expected } => {
                bottom.push(Line::from("Expected:"));
                bottom.push(Line::from(format!("{}", expected)).blue());
                //TODO: show in addition or instead the diff
            }
        }
        bottom.push(Line::default());
    }

    render_train(frame, exo, bottom);
}

// The common top part with exo name and instruction
fn render_common_top(lines: &mut Vec<Line>, exo: &Arc<Exo>) {
    lines.push(Line::from(exo.name.clone()).cyan().bold());
    if let Some(instr) = &exo.instruction {
        // Make sure each line has a real Line because newlines are dropped otherwise
        instr
            .clone()
            .lines()
            .for_each(|l| lines.push(Line::from(l.to_string()).magenta()));
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
