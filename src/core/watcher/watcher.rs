use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::mpsc:: Sender;
use std::time::Duration;
use notify::{EventKind, RecursiveMode, Watcher};
use notify_debouncer_mini::{new_debouncer, DebounceEventResult, DebouncedEventKind};

use crate::{models::event::Event};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FileWatcher{
    // Used for directory or file
    path: PathBuf,
}

impl FileWatcher {
    pub fn new(path: PathBuf) -> Self{
        FileWatcher{path}
    }

    pub fn run(self, tx: Sender<Event>, should_stop: Arc<AtomicBool>){
        let mut debouncer = new_debouncer(Duration::from_secs(30),move |res: DebounceEventResult| {
            match res {
                Ok(events) => {
                    for event in events{
                        if let DebouncedEventKind::Any = event.kind {
                            tx.send(Event::FileSaved).unwrap();
                            break;
                        }
                    }
                }
                Err(_) => { }
            }
        }).unwrap();

        // If the path is a directory, recursive_mode will be evaluated.
        // If recursive_mode is RecursiveMode::Recursive events will be delivered for all files in that tree.
        // If the path is a file, recursive_mode will be ignored and events will be delivered only for the file.
        debouncer.watcher().watch(&self.path, RecursiveMode::Recursive);

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc::{channel, TryRecvError};
    use std::thread;
    use std::fs::OpenOptions;
    use std::io::Write;
    use std::time::Duration;

    /// Tests the creation of a new `FileWatcher` instance.
    #[test]
    fn test_file_watcher_modify_file() {
        let path = PathBuf::from("examples/");
        let (tx, rx) = channel();
        let should_stop = Arc::new(AtomicBool::new(false));
        let watcher = FileWatcher::new(path.clone());

        let should_stop_clone = Arc::clone(&should_stop);
        // Run the file watcher in a separate thread.
        let handel = thread::spawn(move || {
            watcher.clone().run(tx, should_stop_clone);
        });

        thread::sleep(Duration::from_secs(1));

        // Open a file with append option
        let mut data_file = OpenOptions::new()
            .append(true)
            .open("examples/basics/c/basic-args.c")
            .expect("cannot open file");

        // Write to a file
        data_file.write_all("test".as_bytes());
        data_file.flush();

        // Allow debounce time to trigger modification event.
        thread::sleep(Duration::from_secs(3));

        // Check if an event was received.
        match rx.try_recv() {
            Ok(event) => assert_eq!(event, Event::FileSaved),
            Err(TryRecvError::Empty) => panic!("Expected an event but got none"),
            Err(TryRecvError::Disconnected) => panic!("Channel disconnected"),
        }
        handel.join();
    }
}
