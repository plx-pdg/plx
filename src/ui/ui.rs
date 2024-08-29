use std::{
    io::Stdout,
    sync::{Arc, Mutex},
};

use crate::{core::core::PlxCore, models::ui_state::UiState};
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Span, Text},
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
                    self.run(&mut terminal, core.get_state());
                    // if !self.run(&mut terminal, core.get_state()) {
                    //     break;
                    // }
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
            .split(frame.area());

        match state {
            UiState::Home => {
                let title_text = r#"
████████  ██       ██     ██ 
██     ██ ██        ██   ██  
██     ██ ██         ██ ██   
████████  ██          ███    
██        ██         ██ ██   
██        ██        ██   ██  
██        ████████ ██     ██
                    "#;

                let title = Paragraph::new(Text::from(title_text))
                .block(Block::default().borders(Borders::ALL).padding(Padding::new(0, 0, 10, 10)))
                .alignment(ratatui::layout::Alignment::Center);
                let content = Paragraph::new("Press 'r' to resume progress\nPress 'l' to list all Exercices\nPress '?' to display help\n").centered().block(Block::default().borders(Borders::ALL).padding(Padding::new(0, 0, 10, 10)));

                frame.render_widget(title, display[0]);
                frame.render_widget(content, display[1]);
            }
            _ => {}
        }
        return true;
    }

    fn handle_events(&self) -> io::Result<()> {
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
