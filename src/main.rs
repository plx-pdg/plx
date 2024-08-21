//! # [Ratatui] List example
//!
//! The latest version of this example is available in the [examples] folder in the repository.
//!
//! Please note that the examples are designed to be run against the `main` branch of the Github
//! repository. This means that you may not be able to compile with the latest release version on
//! crates.io, or the one that you have installed locally.
//!
//! See the [examples readme] for more information on finding examples that match the version of the
//! library you are using.
//!
//! [Ratatui]: https://github.com/ratatui-org/ratatui
//! [examples]: https://github.com/ratatui-org/ratatui/blob/main/examples
//! [examples readme]: https://github.com/ratatui-org/ratatui/blob/main/examples/README.md

#![allow(clippy::enum_glob_use, clippy::wildcard_imports)]

use core::{Core, Key, UIState};
use std::{
    error::Error,
    io::{self, stdout},
};

pub mod core;
pub mod plx_core;

use color_eyre::config::HookBuilder;
use plx_core::{PlxCore, PlxExercise, PlxSubject};
use ratatui::{
    crossterm::{
        event::{self, Event, KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    prelude::*,
    style::palette::tailwind,
    widgets::*,
};

const BACKGROUND_COLOR: Color = tailwind::BLACK;
const SELECTED_BLOCK_HEADER_COLOR: Color = tailwind::BLUE.c500;
const UNSELECTED_BLOCK_HEADER_COLOR: Color = tailwind::BLACK;
const TODO_HEADER_BG: Color = tailwind::BLUE.c950;
const NORMAL_ROW_COLOR: Color = tailwind::SLATE.c950;
const ALT_ROW_COLOR: Color = tailwind::SLATE.c900;
const SELECTED_STYLE_FG: Color = tailwind::BLUE.c300;
const TEXT_COLOR: Color = tailwind::SLATE.c200;
const COMPLETED_TEXT_COLOR: Color = tailwind::GREEN.c500;

/// This struct holds the current state of the app. In particular, it has the `items` field which is
/// a wrapper around `ListState`. Keeping track of the items state let us render the associated
/// widget with its state and have access to features such as natural scrolling.
///
/// Check the event handling at the bottom to see how to change the state on incoming events.
/// Check the drawing logic for items on how to specify the highlighting style for selected items.
struct Ui {
    core: Box<dyn Core>,
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    init_error_hooks()?;
    let terminal = init_terminal()?;

    // create app and run it
    Ui::new().run(terminal)?;

    restore_terminal()?;

    Ok(())
}

fn init_error_hooks() -> color_eyre::Result<()> {
    let (panic, error) = HookBuilder::default().into_hooks();
    let panic = panic.into_panic_hook();
    let error = error.into_eyre_hook();
    color_eyre::eyre::set_hook(Box::new(move |e| {
        let _ = restore_terminal();
        error(e)
    }))?;
    std::panic::set_hook(Box::new(move |info| {
        let _ = restore_terminal();
        panic(info);
    }));
    Ok(())
}

fn init_terminal() -> color_eyre::Result<Terminal<impl Backend>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal() -> color_eyre::Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

impl Ui {
    fn new() -> Self {
        //TODO this will have to be an Arc<Mutex<PlxCore>>>
        Self {
            core: Box::new(PlxCore::new()),
        }
    }
}

impl Ui {
    //TODO use tryfrom for this
    fn backend_key_to_core_key(key: KeyCode) -> Option<Key> {
        use KeyCode::*;
        match key {
            Char('q') | Esc => Some(Key::Q),
            Char('h') | Left => Some(Key::H),
            Char('j') | Down => Some(Key::J),
            Char('k') | Up => Some(Key::K),
            Char('l') | Right => Some(Key::L),
            Enter => Some(Key::Enter),
            // Char('g') => self.go_top(),
            // Char('G') => self.go_bottom(),
            _ => None,
        }
    }
    fn run(&mut self, mut terminal: Terminal<impl Backend>) -> io::Result<()> {
        loop {
            if self.core.quit() {
                return Ok(());
            }
            self.draw(&mut terminal)?;

            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match Ui::backend_key_to_core_key(key.code) {
                        Some(key) => self.core.on_click(key),
                        None => (),
                    }
                }
            }
        }
    }

    fn draw(&mut self, terminal: &mut Terminal<impl Backend>) -> io::Result<()> {
        terminal.draw(|frame| frame.render_widget(self, frame.area()))?;
        Ok(())
    }
}

impl Widget for &mut Ui {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let bg_block = Block::new().borders(Borders::NONE).bg(BACKGROUND_COLOR);
        bg_block.render(area, buf);

        match self.core.get_state() {
            UIState::Starting => {
                self.render_home_page(area, buf);
            }
            UIState::SelectingSubject(_) | UIState::SelectingExercise(_, _) => {
                let vertical = Layout::vertical([
                    Constraint::Length(2),
                    Constraint::Min(0),
                    Constraint::Length(2),
                ]);
                let [header_area, rest_area, footer_area] = vertical.areas(area);

                let horizontal =
                    Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]);

                let [left_item_area, right_item_area] = horizontal.areas(rest_area);
                render_title(header_area, buf);
                self.render_subjects(left_item_area, buf);
                self.render_exercises(right_item_area, buf);
                // self.render_info(lower_item_list_area, buf);
                render_footer(footer_area, buf);
            }
            UIState::Quit => return,
        }
        // Create a space for header, todo list and the footer.

        // Create two chunks with equal vertical screen space. One for the list and the other for
        // the info block.
    }
}

impl Ui {
    fn render_home_page(&self, area: Rect, buf: &mut Buffer) {
        let vertical = Layout::vertical([
            Constraint::Percentage(80),
            Constraint::Min(1),
            Constraint::Min(1),
            Constraint::Min(1),
        ]);

        let [title_area, resume_command_area, list_command_area, help_command_area] =
            vertical.areas(area);

        Paragraph::new("PLX")
            .bold()
            .centered()
            .blue()
            .on_black()
            .render(title_area, buf);

        Paragraph::new("Press R to Resume where you left off")
            .bold()
            .centered()
            .white()
            .on_black()
            .render(resume_command_area, buf);

        Paragraph::new("Press L to List the available exercises")
            .bold()
            .centered()
            .white()
            .on_black()
            .render(list_command_area, buf);

        Paragraph::new("Press ? to show help")
            .bold()
            .centered()
            .white()
            .on_black()
            .render(help_command_area, buf);
    }
    fn render_exercises(&mut self, area: Rect, buf: &mut Buffer) {
        // We create two blocks, one is for the header (outer) and the other is for list (inner).
        let outer_block =
            Block::new()
                .borders(Borders::NONE)
                .fg(TEXT_COLOR)
                .bg(match self.core.get_state() {
                    UIState::SelectingExercise(_, _) => SELECTED_BLOCK_HEADER_COLOR,
                    _ => UNSELECTED_BLOCK_HEADER_COLOR,
                });

        // We get the inner area from outer_block. We'll use this area later to render the table.
        let outer_area = area;
        let inner_area = outer_block.inner(outer_area);

        // We can render the header in outer_area.
        outer_block.render(outer_area, buf);

        // Iterate through all elements in the `items` and stylize them.
        let (exercises, current_index) = self.core.get_exercises();

        if let Some(exercises) = exercises {
            let items: Vec<ListItem> = exercises
                .iter()
                .enumerate()
                .map(|(i, exercise)| exercise.to_list_item(i, current_index))
                .collect();

            let inner_block = Block::new()
                .borders(Borders::NONE)
                .title_alignment(Alignment::Center)
                .title(format!("{}", items.len()))
                .fg(TEXT_COLOR)
                .bg(match self.core.get_state() {
                    UIState::SelectingExercise(_, _) => SELECTED_BLOCK_HEADER_COLOR,
                    _ => UNSELECTED_BLOCK_HEADER_COLOR,
                });

            // Create a List from all list items and highlight the currently selected one
            let items = List::new(items)
                .block(inner_block)
                .highlight_style(
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .add_modifier(Modifier::REVERSED)
                        .fg(SELECTED_STYLE_FG),
                )
                .highlight_symbol(">>")
                .highlight_spacing(HighlightSpacing::Always);

            // We can now render the item list
            // (look careful we are using StatefulWidget's render.)
            // ratatui::widgets::StatefulWidget::render as stateful_render
            let mut state = ListState::default().with_selected(Some(current_index));
            StatefulWidget::render(items, inner_area, buf, &mut state);
        }
    }
    fn render_subjects(&mut self, area: Rect, buf: &mut Buffer) {
        // We create two blocks, one is for the header (outer) and the other is for list (inner).
        let outer_block =
            Block::new()
                .borders(Borders::NONE)
                .fg(TEXT_COLOR)
                .bg(match self.core.get_state() {
                    UIState::SelectingSubject(_) => SELECTED_BLOCK_HEADER_COLOR,
                    _ => UNSELECTED_BLOCK_HEADER_COLOR,
                });
        // We get the inner area from outer_block. We'll use this area later to render the table.
        let outer_area = area;
        let inner_area = outer_block.inner(outer_area);

        // We can render the header in outer_area.
        outer_block.render(outer_area, buf);

        // Iterate through all elements in the `items` and stylize them.
        let (subjects, current_index) = self.core.get_subjects();

        let items: Vec<ListItem> = subjects
            .iter()
            .enumerate()
            .map(|(i, subject)| subject.to_list_item(i, current_index))
            .collect();

        let inner_block = Block::new()
            .borders(Borders::NONE)
            .title_alignment(Alignment::Center)
            .title(format!("{} / {}", current_index, items.len()))
            .fg(TEXT_COLOR)
            .bg(match self.core.get_state() {
                UIState::SelectingSubject(_) => SELECTED_BLOCK_HEADER_COLOR,
                _ => UNSELECTED_BLOCK_HEADER_COLOR,
            });

        // Create a List from all list items and highlight the currently selected one
        let items = List::new(items)
            .block(inner_block)
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::REVERSED)
                    .fg(SELECTED_STYLE_FG),
            )
            .highlight_symbol(">>")
            .highlight_spacing(HighlightSpacing::Always);

        // We can now render the item list
        // (look careful we are using StatefulWidget's render.)
        // ratatui::widgets::StatefulWidget::render as stateful_render
        let mut state = ListState::default().with_selected(Some(current_index));
        StatefulWidget::render(items, inner_area, buf, &mut state);
    }

    fn render_info(&self, area: Rect, buf: &mut Buffer) {
        // We get the info depending on the item's state.
        let info = if let Some(current_exercise) = self.core.get_current_exercise() {
            current_exercise.prompt.clone()
        } else {
            "Nothing to see here...".to_string()
        };

        // We show the list item's info under the list in this paragraph
        let outer_info_block = Block::new()
            .borders(Borders::NONE)
            .title_alignment(Alignment::Center)
            .title("TODO Info")
            .fg(TEXT_COLOR)
            .bg(TODO_HEADER_BG);
        let inner_info_block = Block::new()
            .borders(Borders::NONE)
            .padding(Padding::horizontal(1))
            .bg(NORMAL_ROW_COLOR);

        // This is a similar process to what we did for list. outer_info_area will be used for
        // header inner_info_area will be used for the list info.
        let outer_info_area = area;
        let inner_info_area = outer_info_block.inner(outer_info_area);

        // We can render the header. Inner info will be rendered later
        outer_info_block.render(outer_info_area, buf);

        let info_paragraph = Paragraph::new(info)
            .block(inner_info_block)
            .fg(TEXT_COLOR)
            .wrap(Wrap { trim: false });

        // We can now render the item info
        info_paragraph.render(inner_info_area, buf);
    }
}

fn render_title(area: Rect, buf: &mut Buffer) {
    Paragraph::new("PLX")
        .bold()
        .left_aligned()
        .render(area, buf);
    Paragraph::new("50%")
        .bold()
        .right_aligned()
        .render(area, buf)
}

fn render_footer(area: Rect, buf: &mut Buffer) {
    Paragraph::new("\nUse ↓↑ to move, ← to unselect, → to change status, g/G to go top/bottom.")
        .centered()
        .render(area, buf);
}

impl PlxSubject {
    fn to_list_item(&self, index: usize, current_index: usize) -> ListItem {
        let bg_color = match index % 2 {
            0 => NORMAL_ROW_COLOR,
            _ => ALT_ROW_COLOR,
        };
        let line = {
            if current_index == index {
                Line::styled(format!("{}", self.title), (COMPLETED_TEXT_COLOR, bg_color))
            } else {
                Line::styled(format!("{}", self.title), TEXT_COLOR)
            }
        };

        ListItem::new(line).bg(bg_color)
    }
}
impl PlxExercise {
    fn to_list_item(&self, index: usize, current_index: usize) -> ListItem {
        let bg_color = match index % 2 {
            0 => NORMAL_ROW_COLOR,
            _ => ALT_ROW_COLOR,
        };
        let line = {
            if current_index == index {
                Line::styled(format!("{}", self.title), (COMPLETED_TEXT_COLOR, bg_color))
            } else {
                Line::styled(format!("{}", self.title), TEXT_COLOR)
            }
        };

        ListItem::new(line).bg(bg_color)
    }
}
