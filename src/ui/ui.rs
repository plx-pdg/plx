use std::{
    io::Stdout,
    sync::{
        atomic::Ordering,
        mpsc::{Receiver, Sender, TryRecvError},
    },
};

use super::{pages::train, utils::ui_key_to_core_key};
use crate::{
    core::work::{work::Work, work_type::WorkType},
    models::ui_state::UiState,
};
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
            UiState::Compiling { exo } => train::render_compilation(frame, &exo),
            UiState::CompileError {
                scroll_offset,
                exo,
                error,
            } => train::render_compilation_error(frame, exo, scroll_offset, error),
            UiState::CheckResults {
                scroll_offset,
                exo,
                checks,
            } => train::render_check_results(frame, exo, scroll_offset, checks),
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
impl Work for Ui {
    fn run(&self, tx: Sender<Event>, stop: std::sync::Arc<std::sync::atomic::AtomicBool>) -> bool {
        let mut local_state: Option<UiState> = None;
        if let Err(err) = self.setup() {
            eprintln!("Couldn't initialize UI {}", err);
            return false;
        }
        let mut terminal = match Terminal::new(CrosstermBackend::new(stdout())) {
            Ok(terminal) => terminal,
            Err(err) => {
                eprintln!("Couldn't setup terminal {}", err);
                return false;
            }
        };

        while !stop.load(Ordering::Relaxed) {
            match self.rx.try_recv() {
                Ok(state) => {
                    local_state = Some(state);
                }
                Err(TryRecvError::Disconnected) => break,
                Err(TryRecvError::Empty) => {}
            }

            if let Some(state) = &local_state {
                let _ = self.tick(&mut terminal, &state, &tx);
            }
        }

        if let Err(err) = self.teardown() {
            eprintln!("Error tearing down UI {}", err);
        }
        return true;
    }

    fn work_type(&self) -> crate::core::work::work_type::WorkType {
        WorkType::Ui
    }
}
