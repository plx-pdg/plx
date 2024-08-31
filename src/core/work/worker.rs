use crate::models::event::Event;

use super::{work::Work, work_handler::WorkEvent};
use std::{
    sync::{atomic::AtomicBool, mpsc::Sender, Arc},
    thread::{self, JoinHandle},
};
pub struct Worker {
    id: usize,
    work_tx: Sender<WorkEvent>,
    pub work_type: Work,
    pub tx: Sender<Event>,
    should_stop: Arc<AtomicBool>,
}

impl Worker {
    pub fn new(
        id: usize,
        work_tx: Sender<WorkEvent>,
        tx: Sender<Event>,
        should_stop: Arc<AtomicBool>,
        work_type: Work,
    ) -> Worker {
        Worker {
            id,
            work_tx,
            tx,
            work_type,
            should_stop,
        }
    }
    pub fn run(self) -> JoinHandle<()> {
        match self.work_type {
            Work::EditorOpen(opener) => {
                return thread::spawn(move || {
                    opener.run(self.tx, self.should_stop);
                    self.work_tx.send(WorkEvent::Done(self.id));
                });
            }
            Work::DirectoryWatcher(watcher) =>{
                return thread::spawn(move || {
                    watcher.run(self.tx, self.should_stop );
                    self.work_tx.send(WorkEvent::Done(self.id));
                })
            }
        }
    }
}
