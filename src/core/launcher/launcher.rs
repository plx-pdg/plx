use std::{
    path::PathBuf,
    sync::{
        atomic::AtomicBool,
        mpsc::{self, Sender},
        Arc,
    },
};

use crate::{
    core::{
        runner::runner::{RunEvent, Runner},
        work::{work::Work, work_type::WorkType},
    },
    models::event::Event,
};

pub struct Launcher {
    id: usize,
    runner: Runner,
}
impl Launcher {
    pub fn new(id: usize, command: PathBuf, args: Vec<String>) -> Option<Self> {
        if let Some(cmd) = command.to_str() {
            Some(Self {
                id,
                runner: Runner::new(String::from(cmd), args),
            })
        } else {
            None
        }
    }
    pub fn get_full_command(&self) -> String {
        self.runner.get_full_command()
    }
}
impl Work for Launcher {
    fn run(&self, tx: Sender<Event>, stop: Arc<AtomicBool>) -> bool {
        let (runner_tx, runner_rx) = mpsc::channel();
        let _ = self.runner.run(runner_tx, stop);
        while let Ok(msg) = runner_rx.recv() {
            let send = match msg {
                RunEvent::ProcessCreationFailed(_) => {
                    return false;
                }
                RunEvent::ProcessCreated => tx.send(Event::RunStart(self.id)),
                RunEvent::ProcessEnd(_) => tx.send(Event::RunEnd(self.id)),
                RunEvent::ProcessNewOutputLine(line) => {
                    tx.send(Event::RunOutputLine(self.id, line))
                }
            };
            if send.is_err() {
                break;
            }
        }
        return true;
    }

    fn work_type(&self) -> WorkType {
        WorkType::Launcher
    }
}
