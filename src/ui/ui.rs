use std::{
    io::Stdout,
    sync::{Mutex, Weak},
};

use crate::{
    core::core::PlxCore,
    models::{event, ui_state::UiState},
};
use rand::Error;
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, Event, KeyCode},
        terminal::{
            self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
        },
        ExecutableCommand,
    },
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    symbols::{block, line::BOTTOM_LEFT},
    text::{Span, Text},
    widgets::{Block, Borders, Paragraph},
    DefaultTerminal, Frame, Terminal,
};
use std::{
    io::{self, stdout},
    ops::RangeBounds,
    slice::Chunks,
};

pub struct Ui<'a> {
    core: Weak<Mutex<PlxCore<'a>>>,
}
impl Ui<'_> {
    pub fn new(core: Weak<Mutex<PlxCore>>) -> Ui<'_> {
        Ui { core }
    }
    fn setup(&mut self)  -> io::Result<()> {
        println!("Ui Setup...");
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        Ok(())
    }
    fn teardown(&mut self)  -> io::Result<()> {
        println!("Ui Teardown...");
        disable_raw_mode()?;
        stdout().execute(LeaveAlternateScreen)?;
        Ok(())
    }
    pub fn loop_forever(&mut self) -> io::Result<()> {
        self.setup()?;
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

        loop {
            match self.core.upgrade() {
                Some(core) => {
                    if let Ok(core) = core.lock() {
                        self.run(&mut terminal, core.get_state());
                        // if !self.run(&mut terminal, core.get_state()) {
                        //     break;
                        // }
                    }
                }
                None => break,
            }
        }
        self.teardown()?;
        Ok(())
    }

    pub fn run(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
        state: &UiState,
    ) -> Result<bool, io::Error> {
        terminal.draw(|frame| {
            if !self.render_frame(frame, state) {
                // return Ok(false);
            }
        })?;
        self.handle_events()?;
        Ok(true)
    }

    fn render_frame(&self, frame: &mut Frame, state: &UiState) -> bool {
        let display = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(frame.size());

        match state {
            UiState::Home => {
                let title = Paragraph::new(Text::from(Span::styled(
                    "PLX",
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .add_modifier(Modifier::UNDERLINED),
                )))
                .block(Block::default().borders(Borders::ALL))
                .alignment(ratatui::layout::Alignment::Center);

                let content = Paragraph::new("Press 'r' to resume progress\nPress 'l' to list all Exercices\nPress '?' to display help\n").centered().block(Block::default().borders(Borders::ALL));

                frame.render_widget(title, display[0]);
                frame.render_widget(content, display[1]);
            }
            _ => {}
        }
        return true;
    }

    fn handle_events(&mut self) -> io::Result<()> {
        todo!()
    }

    // fn handle_events(app_state: &mut AppState) -> io::Result<bool> {
    //     if event::poll(std::time::Duration::from_millis(50))? {
    //         if let Event::Key(key) = event::read()? {
    //             //TODO send event to the core
    //         }
    //     }
    //     Ok(false)
    // }
}
