use std::sync::{atomic::AtomicBool, mpsc::Sender, Arc};

use crate::models::event::Event;

use super::work_type::WorkType;

pub trait Work {
    /// This function has the right to block
    /// but _should_ stop once stop is set to true
    /// returns false only if setup went wrong else return true
    fn run(&self, tx: Sender<Event>, stop: Arc<AtomicBool>) -> bool;

    /// returns this work's type
    fn work_type(&self) -> WorkType;
}
