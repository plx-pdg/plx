use std::{
    io::Stdout,
    path,
    sync::{Arc, Mutex},
};

use crate::{
    core::core::PlxCore,
    models::ui_state::{self, UiState},
    ui::pages::home,
};

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, Event, KeyCode},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use std::io::{self, stdout};

pub struct Ui<'a> {
    core: Arc<Mutex<PlxCore<'a>>>,
}
impl Ui<'_> {
    pub fn new(core: Arc<Mutex<PlxCore>>) -> Ui<'_> {
        Ui { core }
    }
    fn setup(&mut self) -> io::Result<()> {
        println!("Ui Setup...");
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        Ok(())
    }
    fn teardown(&mut self) -> io::Result<()> {
        println!("Ui Teardown...");
        disable_raw_mode()?;
        stdout().execute(LeaveAlternateScreen)?;
        Ok(())
    }
    pub fn loop_forever(&mut self) -> io::Result<()> {
        self.setup()?;
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

        loop {
            match self.core.lock() {
                Ok(core) => {
                    // self.run(&mut terminal, core.get_state());
                    // // if !self.run(&mut terminal, core.get_state()) {
                    // //     break;
                    // // }
                    if !self.run(&mut terminal, core.get_state())? {
                        break;
                    }
                }
                Err(_) => break,
            }
        }
        self.teardown()?;
        Ok(())
    }

    pub fn run(
        &self,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
        state: &UiState,
    ) -> Result<bool, io::Error> {
        terminal.draw(|frame| {
            if !self.render_frame(frame, state) {
                // return Ok(false);
                self.render_frame(frame, state);
            }
        })?;
        if !self.handle_events()? {
            return Ok(false);
        };
        Ok(true)
    }

    fn render_frame(&self, frame: &mut Frame, state: &UiState) -> bool {
        match state {
            UiState::Home => {
                home::render_home(frame);
                //TODO all other pages
            }
            _ => {}
        }
        return true;
    }

    fn handle_events(&self /*, ui_state aussi pour render les pages*/) -> io::Result<bool> {
        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(false),

                    _ => {}
                }
            }
        }
        Ok(true)
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
