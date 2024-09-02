use crate::core::editor::opener::EditorOpener;
use crate::core::watcher::watcher::FileWatcher;
use crate::models::event::Event;
use std::sync::{atomic::AtomicBool, mpsc::Sender, Arc};

use super::work_type::WorkType;

pub trait Work {
    /// This function has the right to block
    /// but _should_ stop once stop is set to true
    /// returns false only if setup went wrong else return true
    fn run(&self, tx: Sender<Event>, stop: Arc<AtomicBool>) -> bool;

    /// returns this work's type
    fn work_type(&self) -> WorkType;
}
