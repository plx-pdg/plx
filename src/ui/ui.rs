use std::{
    io::Stdout,
    sync::mpsc::{Receiver, Sender, TryRecvError},
};

use super::utils::ui_key_to_core_key;
use crate::models::ui_state::UiState;
use crate::{models::event::Event, ui::pages::home};
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, Event as CrosstermEvent},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    Frame, Terminal,
};
use std::io::{self, stdout};

pub struct Ui {
    rx: Receiver<UiState>,
}
impl Ui {
    pub fn new(rx: Receiver<UiState>) -> Ui {
        Ui { rx }
    }
    fn setup(&self) -> io::Result<()> {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        Ok(())
    }
    fn teardown(&self) -> io::Result<()> {
        disable_raw_mode()?;
        stdout().execute(LeaveAlternateScreen)?;
        Ok(())
    }
    pub fn loop_forever(&mut self) -> io::Result<()> {
        let mut local_state: Option<UiState> = None;
        self.setup()?;
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

        loop {
            match self.rx.try_recv() {
                Ok(state) => {
                    local_state = Some(state); //TODO: how performant is it ??
                }
                Err(TryRecvError::Disconnected) => break,
                Err(TryRecvError::Empty) => {}
            }

            if let Some(state) = &local_state {
                self.run(&mut terminal, &state);
            }
        }
        self.teardown()?;
        Ok(())
    }

    pub fn tick(
        &self,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
        state: &UiState,
        tx: &Sender<Event>,
    ) -> Result<(), io::Error> {
        terminal.draw(|frame| self.render_frame(frame, state))?;
        Ui::handle_events(tx)
    }

    fn render_frame(&self, frame: &mut Frame, state: &UiState) {
        match state {
            UiState::Home => home::render_home(frame),
            UiState::Quit => return,
            //TODO all other pages
            _ => {}
        }
    }

    fn handle_events(tx: &Sender<Event>) -> io::Result<()> {
        if event::poll(std::time::Duration::from_millis(50))? {
            if let CrosstermEvent::Key(key) = event::read()? {
                if let Some(k) = ui_key_to_core_key(&key.code) {
                    let _ = tx.send(Event::KeyPressed(k));
                }
            }
        }
        Ok(())
    }
}
