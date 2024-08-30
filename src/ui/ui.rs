use std::{
    io::Stdout,
    sync::mpsc::{Receiver, Sender, TryRecvError},
};

use crate::models::ui_state::UiState;
use crate::{models::event::Event, models::key::Key, ui::pages::home};
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, Event as CrosstermEvent, KeyCode},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    Frame, Terminal,
};
use std::io::{self, stdout};

// TODO: is it the correct place to put this function ?
fn ui_key_to_core_key(key: &KeyCode) -> Option<Key> {
    match key {
        KeyCode::Char('q') => Some(Key::Q),
        KeyCode::Char('h') | KeyCode::Left => Some(Key::H),
        KeyCode::Char('j') | KeyCode::Down => Some(Key::J),
        KeyCode::Char('k') | KeyCode::Up => Some(Key::K),
        KeyCode::Char('l') | KeyCode::Right => Some(Key::L),
        KeyCode::Enter => Some(Key::Enter),
        KeyCode::Esc => Some(Key::Esc),
        _ => None,
    }
}

pub struct Ui<'a> {
    tx: Sender<Event>,
    rx: Receiver<UiState<'a>>,
}
impl Ui<'_> {
    pub fn new(tx: Sender<Event>, rx: Receiver<UiState>) -> Ui<'_> {
        Ui { tx, rx }
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
            UiState::Home => home::render_home(frame),
            UiState::Quit => return false, //TODO: this is the way we try to quit for now
            //TODO all other pages
            _ => {}
        }
        true
    }

    fn handle_events(&self /*, ui_state aussi pour render les pages*/) -> io::Result<bool> {
        if event::poll(std::time::Duration::from_millis(50))? {
            if let CrosstermEvent::Key(key) = event::read()? {
                if let Some(k) = ui_key_to_core_key(&key.code) {
                    self.tx.send(Event::KeyPressed(k));
                }
            }
        }
        Ok(true)
    }
}
