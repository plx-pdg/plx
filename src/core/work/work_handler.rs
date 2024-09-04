use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

use log::info;

use crate::models::event::Event;

use super::{
    work::Work,
    work_type::WorkType,
    worker::{self, Worker},
};

pub(crate) enum WorkEvent {
    Done(usize),
    Stop(usize),
}
struct WorkInfo {
    id: usize,
    work: WorkType,
    should_stop: Arc<AtomicBool>,
    join_handle: JoinHandle<()>,
}

pub struct WorkHandler {
    workers: HashMap<usize, WorkInfo>,
    tx: Sender<Event>,
    work_tx: Sender<WorkEvent>,
    work_id: usize,
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
            work_id: 0,
        }));
        WorkHandler::run(ret.clone(), rx);
        ret
    }

    pub fn spawn_worker(&mut self, work: Box<dyn Work + Send>) -> usize {
        let id = self.work_id;
        self.work_id += 1;
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
    pub fn stop_workers(&mut self, work_type: WorkType) {
        self.workers
            .iter_mut()
            .filter(|(_, worker)| worker.work == work_type)
            .for_each(|(_, worker)| worker.stop());
        self.workers.retain(|_, worker| worker.work != work_type);
    }
    fn stop_all_workers(&mut self) {
        self.workers
            .iter_mut()
            .for_each(|(_, worker)| worker.stop());
    }

    pub fn stop_all_workers_and_wait(&mut self) {
        self.stop_all_workers(); // Signal every worker to stop
        let workers: Vec<_> = self.workers.drain().map(|(_, worker)| worker).collect();

        for worker in workers {
            info!("Join {} {:#?}", worker.id, worker.work);
            worker.join(); // join here
        }
    }

    fn remove_worker(&mut self, id: usize) {
        info!("Remove {}", id);
        self.workers.remove(&id);
    }
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
