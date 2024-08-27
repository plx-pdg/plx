use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::Sender;
use notify::{Config, RecommendedWatcher, EventKind, RecursiveMode, Watcher};

use crate::{
    core::process::process_handler::{self, wait_child},
    models::{event::Event, exo::Exo}
};
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FileWatcher{
    path: PathBuf,
    exo: Exo
}

impl FileWatcher {
    pub fn new(path: PathBuf, exo: Exo) -> Self{
        FileWatcher{path, exo}
    }
    pub fn run(&self, tx: Sender<Event>){
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

        let (sx, rx) = std::sync::mpsc::channel();

        let mut watcher = RecommendedWatcher::new(sx, Config::default());

        // Add a path to be watched. All files and directories at that path and
        // below will be monitored for changes.
        watcher.unwrap().watch(self.path.as_ref(), RecursiveMode::Recursive);


            match rx[0] {
                Ok(event) => {
                    if let EventKind::Modify(modify_kind) = event.kind {
                        if matches!(modify_kind, notify::event::ModifyKind::Data(_)) {
                            tx.send(Event::FileSaved);
                        }
                    }
                } ,
                Err(_) => {tx.send(Event::FileNotSaved);} ,
            };
    }
}
