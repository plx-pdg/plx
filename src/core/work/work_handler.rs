use std::{collections::HashMap, sync::mpsc::Sender};

use crate::models::event::Event;

use super::{work::Work, worker::Worker};

pub struct WorkHandler {
    workers: HashMap<usize, Worker>,
    tx: Sender<Event>,
}
impl WorkHandler {
    pub fn new(tx: Sender<Event>) -> Self {
        WorkHandler {
            workers: HashMap::new(),
            tx,
        }
    }
    fn spawn_id(&self) -> usize {
        loop {
            let id = rand::random::<usize>();
            if !self.workers.contains_key(&id) {
                return id;
            }
        }
    }
    pub fn spawn_worker(&mut self, work_type: Work) -> usize {
        let id = self.spawn_id();
        let mut worker = Worker::new(self.tx.clone(), work_type);
        worker.run();
        self.workers.insert(id, worker);
        id
    }
    pub fn stop_worker(&mut self, id: usize) {
        match self.workers.remove(&id) {
            Some(mut worker) => worker.stop(),
            None => (),
        }
    }
    pub fn stop_workers(&mut self, work_type: Work) {
        self.workers
            .iter_mut()
            .filter(|(_, worker)| worker.work_type == work_type)
            .for_each(|(_, worker)| worker.stop());
        self.workers
            .retain(|_, worker| worker.work_type != work_type);
    }
    pub fn stop_all_workers(&mut self) {
        self.workers
            .iter_mut()
            .for_each(|(_, worker)| worker.stop());
        self.workers.clear();
    }
}
#[cfg(test)]
mod tests {
    use std::sync::mpsc::channel;

    use super::*;
    use crate::core::editor::opener::EditorOpener;
    #[test]
    fn test_different_ids() {
        let (tx, _) = channel();
        let mut handler = WorkHandler::new(tx.clone());
        let mut last_id: Option<usize> = None;
        for _ in 0..10 {
            let id = handler.spawn_worker(Work::EditorOpen(EditorOpener::new(
                "echo".to_string(),
                "null".into(),
            )));
            if last_id.is_none() {
                last_id = Some(id);
            } else {
                assert_ne!(last_id.unwrap(), id);
            }
        }
    }
    #[test]
    fn test_spawn_worker() {
        let (tx, _) = channel();
        let mut handler = WorkHandler::new(tx.clone());

        let id = handler.spawn_worker(Work::EditorOpen(EditorOpener::new(
            "echo".to_string(),
            "null".into(),
        )));
        assert!(handler.workers.contains_key(&id));
        matches!(
            handler.workers.get(&id).unwrap().work_type,
            Work::EditorOpen(_)
        );
    }
    #[test]
    fn test_stop_worker() {
        let (tx, _) = channel();
        let mut handler = WorkHandler::new(tx.clone());

        let id = handler.spawn_worker(Work::EditorOpen(EditorOpener::new(
            "echo".to_string(),
            "null".into(),
        )));
        //Can't actually check this calls worker.stop() unless worker becomes a trait
        handler.stop_worker(id);
        assert!(!handler.workers.contains_key(&id));
    }

    #[test]
    fn test_stop_all_workers() {
        let (tx, _) = channel();
        let mut handler = WorkHandler::new(tx.clone());

        let range = 10;
        for _ in 0..range {
            let _ = handler.spawn_worker(Work::EditorOpen(EditorOpener::new(
                "echo".to_string(),
                "null".into(),
            )));
        }
        assert_eq!(handler.workers.len(), range);
        //Can't actually check this calls worker.stop() unless worker becomes a trait
        handler.stop_all_workers();
        assert_eq!(handler.workers.len(), 0);
    }
}
