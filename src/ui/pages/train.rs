/// Train page rendering functions
/// Each UiState related to this page has its own function and reuse render_train()
/// that generate the render_common_top() to show exo metadata that should be always visible
use std::sync::Arc;

use crate::models::{
    check::CheckTest,
    check_state::{CheckState, CheckStatus},
    exo::Exo,
};
use ansi_to_tui::IntoText;
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

/// Push lines with the given string taking \n as splitter, or push a "<empty>" dim line when empty
/// We can define a prefix in front of the line
pub fn push_each_line_or_empty_info(lines: &mut Vec<Line>, prefix: String, text: String) {
    let mut prefix = Line::from(prefix);
    prefix.push_span(": ");
    if text.is_empty() {
        prefix.push_span(Span::from("<empty>").dim());
        lines.push(prefix);
    } else {
        if text.find("\n").is_some() {
            lines.push(prefix);
            text.lines()
                .for_each(|l| lines.push(Line::from(l.to_string()).cyan()));
        } else {
            prefix.push_span(Span::from(text).light_blue());
            lines.push(prefix);
        }
    }
}

/// Basic cleanup of absolute path in compilation outputs
fn cleanup_compilation_output(exo: &Arc<Exo>, error: &String) -> String {
    let mut exo_folder_path = exo
        .folder
        .clone()
        .into_os_string()
        .into_string()
        .unwrap_or_default();
    exo_folder_path.push_str("/");
    error.replace(&exo_folder_path, "")
}

/// Use ansi_to_tui to generate Text and push its Line to given lines vector
fn push_ansi_content(lines: &mut Vec<Line>, ansi_string: String, offset: usize) {
    if let Ok(text) = ansi_string.into_text() {
        text.lines
            .iter()
            .skip(offset)
            .for_each(|l| lines.push(l.clone()));
    }
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
    push_ansi_content(
        &mut bottom,
        cleanup_compilation_output(exo, error),
        *scroll_offset,
    );

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
        match check_state.check.test.clone() {
            CheckTest::Output { .. } => match check_state.status.clone() {
                CheckStatus::Passed => {}
                CheckStatus::Failed(expected, output, diff) => {
                    if !check_state.check.args.is_empty() {
                        let mut l = Line::from("Args: ");
                        l.push_span(
                            Span::from(format!("{:?}", check_state.check.args)).light_blue(),
                        );
                        bottom.push(l);
                    }
                    // If one value is empty, maybe showing a diff is not useful
                    if output.trim().is_empty() || expected.trim().is_empty() {
                        push_each_line_or_empty_info(&mut bottom, "Output".to_string(), output);
                        push_each_line_or_empty_info(&mut bottom, "Expected".to_string(), expected);
                    } else {
                        // otherwise, the diff is better
                        bottom.push(Line::from("Diff:"));
                        push_ansi_content(&mut bottom, diff.to_ansi_colors(), 0);
                    }
                }
                CheckStatus::Pending | CheckStatus::Checking => {}
                CheckStatus::Running => {
                    bottom.push(Line::from("Running check...").dim());
                }
                CheckStatus::RunFail(err) => {
                    bottom.push(Line::from(format!("Running the check has failed: {}", err)).dim());
                }
            },
        }
        bottom.push(Line::default());
    }

    render_train(frame, exo, bottom);
}

// The common top part with exo name and instruction
pub fn render_exo(lines: &mut Vec<Line>, exo: &Arc<Exo>, include_exo_files: bool) {
    lines.push(Line::from(exo.name.clone()).cyan().bold());
    if let Some(instr) = &exo.instruction {
        // Make sure each line has a real Line because newlines are dropped otherwise
        instr
            .clone()
            .lines()
            .for_each(|l| lines.push(Line::from(l.to_string()).magenta()));
    }
    if !include_exo_files {
        return;
    }
    lines.push(Line::default());
    lines.push(Line::from("Exo files").cyan().bold());
    exo.files.clone().iter().for_each(|f| {
        lines.push(Line::from(
            f.file_name()
                .unwrap_or_default()
                .to_os_string() // TODO: why do we need to do this ? We unwrapped
                // Option<OsStr> and we don't have access to into_string() ????
                .into_string()
                .unwrap_or_default(), //TODO: how can we do this in less code ??
        ))
    });
}

// Render final page with the common top and specific bottom
fn render_train(frame: &mut Frame, exo: &Arc<Exo>, bottom: Vec<Line>) {
    let mut lines: Vec<Line> = vec![];
    render_exo(&mut lines, exo, false);
    lines.push(Line::default());
    bottom.iter().for_each(|l| lines.push(l.clone()));
    let zone = Paragraph::new(Text::from(lines)).wrap(Wrap { trim: true });
    frame.render_widget(zone, frame.area());
}
