use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

use crate::models::event::Event;

use super::{work::Work, worker::Worker};

pub(crate) enum WorkEvent {
    Done(usize),
    Stop(usize),
}
struct WorkInfo {
    work: Work,
    should_stop: Arc<AtomicBool>,
    join_handle: JoinHandle<()>,
}

pub struct WorkHandler {
    workers: HashMap<usize, WorkInfo>,
    tx: Sender<Event>,
    work_tx: Sender<WorkEvent>,
}
impl WorkInfo {
    fn new(work: Work, should_stop: Arc<AtomicBool>, join_handle: JoinHandle<()>) -> Self {
        WorkInfo {
            work,
            should_stop,
            join_handle,
        }
    }
    fn stop(&mut self) {
        self.should_stop.store(true, Ordering::Relaxed);
    }
    fn join(self) {
        self.join_handle.join();
    }
    fn stop_and_join(mut self) {
        self.stop();
        self.join();
    }
}
impl WorkHandler {
    pub fn new(tx: Sender<Event>) -> Arc<Mutex<Self>> {
        let (work_tx, rx) = mpsc::channel();
        let ret = Arc::new(Mutex::new(WorkHandler {
            workers: HashMap::new(),
            tx,
            work_tx,
        }));
        WorkHandler::run(ret.clone(), rx);
        ret
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
        let stop = Arc::new(AtomicBool::new(false));
        let worker = Worker::new(
            id,
            self.work_tx.clone(),
            self.tx.clone(),
            stop.clone(),
            work_type.clone(),
        );
        let join_handle = worker.run();
        self.workers
            .insert(id, WorkInfo::new(work_type, stop, join_handle));
        id
    }

    fn run(handler: Arc<Mutex<WorkHandler>>, rx: Receiver<WorkEvent>) {
        thread::spawn(move || {
            while let Ok(msg) = rx.recv() {
                if let Ok(mut handler) = handler.lock() {
                    match msg {
                        WorkEvent::Done(id) => handler.remove_worker(id),
                        WorkEvent::Stop(id) => handler.stop_worker(id),
                    }
                };
            }
        });
    }

    pub fn stop_worker(&mut self, id: usize) {
        match self.workers.remove(&id) {
            Some(worker) => worker.stop_and_join(),
            None => (),
        }
    }
    pub fn stop_workers(&mut self, work_type: Work) {
        self.workers
            .iter_mut()
            .filter(|(_, worker)| worker.work == work_type)
            .for_each(|(_, worker)| worker.stop());
        self.workers.retain(|_, worker| worker.work != work_type);
    }
    pub fn stop_all_workers(&mut self) {
        self.workers
            .iter_mut()
            .for_each(|(_, worker)| worker.stop());
        self.workers.clear();
    }
    fn remove_worker(&mut self, id: usize) {
        self.workers.remove(&id);
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
        let handler = WorkHandler::new(tx.clone());
        let mut last_id: Option<usize> = None;
        for _ in 0..10 {
            if let Ok(mut handler) = handler.lock() {
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
    }
    #[test]
    fn test_spawn_worker() {
        let (tx, _) = channel();
        let handler = WorkHandler::new(tx.clone());

        if let Ok(mut handler) = handler.lock() {
            let id = handler.spawn_worker(Work::EditorOpen(EditorOpener::new(
                "echo".to_string(),
                "null".into(),
            )));

            assert!(handler.workers.contains_key(&id));
            assert!(!handler
                .workers
                .get(&id)
                .unwrap()
                .should_stop
                .load(Ordering::Relaxed));
            assert!(matches!(
                handler.workers.get(&id).unwrap().work,
                Work::EditorOpen(_)
            ));
        };
    }
    #[test]
    fn test_stop_worker() {
        let (tx, _) = channel();
        let handler = WorkHandler::new(tx.clone());

        if let Ok(mut handler) = handler.lock() {
            let id = handler.spawn_worker(Work::EditorOpen(EditorOpener::new(
                "echo".to_string(),
                "null".into(),
            )));
            let stop = handler.workers.get(&id).unwrap().should_stop.clone();
            handler.stop_worker(id);
            assert!(!handler.workers.contains_key(&id));
            assert!(stop.load(Ordering::Relaxed));
        };
    }

    #[test]
    fn test_stop_all_workers() {
        let (tx, _) = channel();
        let handler = WorkHandler::new(tx.clone());

        if let Ok(mut handler) = handler.lock() {
            let range = 10;
            for _ in 0..range {
                let _ = handler.spawn_worker(Work::EditorOpen(EditorOpener::new(
                    "echo".to_string(),
                    "null".into(),
                )));
            }
            let stop_flags: Vec<Arc<AtomicBool>> = handler
                .workers
                .values()
                .into_iter()
                .map(|work_info| work_info.should_stop.clone())
                .collect();
            assert_eq!(handler.workers.len(), range);
            handler.stop_all_workers();
            for flag in stop_flags {
                assert!(flag.load(Ordering::Relaxed));
            }
            assert_eq!(handler.workers.len(), 0);
        };
    }
}
