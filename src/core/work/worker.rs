use crate::models::event::Event;

use super::work::Work;
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::Sender,
        Arc,
    },
    thread::{self, JoinHandle},
};
pub struct Worker {
    pub work_type: Work,
    pub tx: Sender<Event>,
    should_stop: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
}

impl Worker {
    pub fn new(tx: Sender<Event>, work_type: Work) -> Worker {
        Worker {
            tx,
            work_type,
            should_stop: Arc::new(AtomicBool::new(false)),
            handle: None,
        }
    }
    pub fn run(&mut self) {
        match &self.work_type {
            Work::EditorOpen(opener) => {
                let opener = opener.clone();
                let tx = self.tx.clone();
                let should_stop = self.should_stop.clone();
                self.handle = Some(thread::spawn(move || opener.run(tx, should_stop)));
            }
        }
    }
    pub fn stop(&mut self) {
        self.should_stop.store(true, Ordering::Relaxed);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
    pub fn is_stop(&self) -> bool {
        self.should_stop.load(Ordering::Relaxed)
    }
}
#[cfg(test)]
mod tests {
    use std::sync::mpsc::channel;

    use crate::core::editor::opener::EditorOpener;

    use super::*;

    #[test]
    fn test_stop() {
        let (tx, _) = channel();
        let mut worker = Worker::new(
            tx,
            Work::EditorOpen(EditorOpener::new("echo".into(), "hello".into())),
        );
        assert_eq!(worker.should_stop.load(Ordering::Relaxed), false);
        worker.stop();
        assert_eq!(worker.should_stop.load(Ordering::Relaxed), true);
    }
    #[test]
    fn test_handler() {
        let (tx, _) = channel();
        let mut worker = Worker::new(
            tx,
            Work::EditorOpen(EditorOpener::new("echo".into(), "hello".into())),
        );
        assert!(worker.handle.is_none());
        worker.run();
        assert!(worker.handle.is_some());
        worker.stop();
        assert!(worker.handle.is_none());
    }
}
