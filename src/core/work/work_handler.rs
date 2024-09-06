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

use super::{work::Work, work_type::WorkType, worker::Worker};

pub(crate) enum WorkEvent {
    Done(usize),
}

/// Represents all information we have about a specific worker
struct WorkInfo {
    id: usize,
    work: WorkType,
    should_stop: Arc<AtomicBool>,
    join_handle: JoinHandle<()>,
}

/// Work Handler is used to handle every worker the app constructs
pub struct WorkHandler {
    workers: HashMap<usize, WorkInfo>,
    tx: Sender<Event>,
    work_tx: Sender<WorkEvent>,
    curr_work_id: usize,
}
impl WorkInfo {
    fn new(
        id: usize,
        work: WorkType,
        should_stop: Arc<AtomicBool>,
        join_handle: JoinHandle<()>,
    ) -> Self {
        WorkInfo {
            id,
            work,
            should_stop,
            join_handle,
        }
    }
    /// Stop a worker
    fn stop(&mut self) {
        self.should_stop.store(true, Ordering::Relaxed);
    }
    /// Join a worker thread
    fn join(self) {
        let _ = self.join_handle.join();
    }

    /// Stop and join a worker thread
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
            curr_work_id: 0,
        }));
        WorkHandler::run(ret.clone(), rx);
        ret
    }

    /// Spawns a new worker
    /// It can spawn anything that implements `Work` and is `Send`
    pub fn spawn_worker(&mut self, work: Box<dyn Work + Send>) -> usize {
        let id = self.curr_work_id;
        self.curr_work_id += 1;
        let stop = Arc::new(AtomicBool::new(false));
        let work_type = work.work_type();
        let worker = Worker::new(
            id,
            self.work_tx.clone(),
            self.tx.clone(),
            stop.clone(),
            work,
        );
        let join_handle = worker.run_on_separate_thread();
        self.workers
            .insert(id, WorkInfo::new(id, work_type, stop, join_handle));
        id
    }

    /// Launch a separate thread to handle worker events
    /// Basically used to remove done workers from the worker list
    fn run(handler: Arc<Mutex<WorkHandler>>, rx: Receiver<WorkEvent>) {
        thread::spawn(move || {
            while let Ok(msg) = rx.recv() {
                if let Ok(mut handler) = handler.lock() {
                    match msg {
                        WorkEvent::Done(id) => handler.remove_worker(id),
                    }
                };
            }
        });
    }

    /// Stop the worker
    /// This function blocks until the worker actually ends its work
    pub fn stop_worker(&mut self, id: usize) {
        match self.workers.remove(&id) {
            Some(worker) => worker.stop_and_join(),
            None => (),
        }
    }
    /// Stop all workers doing any work of type work_type
    pub fn stop_workers(&mut self, work_type: WorkType) {
        self.workers
            .iter_mut()
            .filter(|(_, worker)| worker.work == work_type)
            .for_each(|(_, worker)| worker.stop());
        self.workers.retain(|_, worker| worker.work != work_type);
    }

    /// Send a stop signal to all workers
    fn stop_all_workers(&mut self) {
        self.workers
            .iter_mut()
            .for_each(|(_, worker)| worker.stop());
    }

    /// Stop and wait for all workers to end
    pub fn stop_all_workers_and_wait(&mut self) {
        self.stop_all_workers(); // Signal every worker to stop
        let workers: Vec<_> = self.workers.drain().map(|(_, worker)| worker).collect();

        for worker in workers {
            worker.join(); // join here
        }
    }

    /// Remove a worker from the list
    /// Worker should have finished before calling this function
    fn remove_worker(&mut self, id: usize) {
        self.workers.remove(&id);
    }

    /// Helper function so we can stop all workers except for the UI one
    /// Very useful when changing exos and having to stop every worker working on the old one
    pub fn clean_non_ui_workers(&mut self) {
        let ids_to_remove: Vec<usize> = self
            .workers
            .iter()
            .filter_map(|(id, info)| {
                if info.work == WorkType::Ui {
                    return None;
                } else {
                    return Some(*id);
                }
            })
            .collect();
        for id in ids_to_remove {
            self.stop_worker(id);
        }
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
                let id = handler.spawn_worker(Box::new(
                    EditorOpener::new("echo".to_string(), "null".into()).unwrap(),
                ));
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
            let id = handler.spawn_worker(Box::new(
                EditorOpener::new("echo".to_string(), "null".into()).unwrap(),
            ));

            assert!(handler.workers.contains_key(&id));
            assert!(!handler
                .workers
                .get(&id)
                .unwrap()
                .should_stop
                .load(Ordering::Relaxed));
            assert!(matches!(
                handler.workers.get(&id).unwrap().work,
                WorkType::EditorOpen
            ));
        };
    }
    #[test]
    fn test_stop_worker() {
        let (tx, _) = channel();
        let handler = WorkHandler::new(tx.clone());

        if let Ok(mut handler) = handler.lock() {
            let id = handler.spawn_worker(Box::new(
                EditorOpener::new("echo".to_string(), "null".into()).unwrap(),
            ));
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
                let _ = handler.spawn_worker(Box::new(
                    EditorOpener::new("echo".to_string(), "null".into()).unwrap(),
                ));
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
            assert_eq!(handler.workers.len(), 10);
        };
    }

    #[test]
    fn test_stop_all_workers_wait() {
        let (tx, _) = channel();
        let handler = WorkHandler::new(tx.clone());

        if let Ok(mut handler) = handler.lock() {
            let range = 10;
            for _ in 0..range {
                let _ = handler.spawn_worker(Box::new(
                    EditorOpener::new("echo".to_string(), "null".into()).unwrap(),
                ));
            }
            let stop_flags: Vec<Arc<AtomicBool>> = handler
                .workers
                .values()
                .into_iter()
                .map(|work_info| work_info.should_stop.clone())
                .collect();
            assert_eq!(handler.workers.len(), range);
            handler.stop_all_workers_and_wait();
            for flag in stop_flags {
                assert!(flag.load(Ordering::Relaxed));
            }
            assert_eq!(handler.workers.len(), 0);
        };
    }
}
