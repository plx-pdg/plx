use notify::RecursiveMode;
use notify::Watcher;
use notify_debouncer_mini::{new_debouncer, DebounceEventResult, DebouncedEventKind};
use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

use crate::models::event::Event;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FileWatcher {
    // Used for directory or file
    path: PathBuf,
}

impl FileWatcher {
    pub fn new(path: PathBuf) -> Self {
        FileWatcher { path }
    }

    /// Runs a directory watcher
    /// This functions blocks and should be called from a different thread
    /// Returns false if init failed else true
    /// TODO standardize worker interface
    pub fn run(self, tx: Sender<Event>, should_stop: Arc<AtomicBool>) -> bool {
        let debouncer =
            new_debouncer(
                Duration::from_secs(1),
                move |res: DebounceEventResult| match res {
                    Ok(events) => {
                        for event in events {
                            println!("{:?}", event);
                            if let DebouncedEventKind::Any = event.kind {
                                tx.send(Event::FileSaved).unwrap();
                                break;
                            }
                        }
                    }
                    Err(_) => {}
                },
            );

        if let Ok(mut debouncer) = debouncer {
            // If the path is a directory, recursive_mode will be evaluated.
            // If recursive_mode is RecursiveMode::Recursive events will be delivered for all files in that tree.
            // If the path is a file, recursive_mode will be ignored and events will be delivered only for the file.
            let watcher = debouncer
                .watcher()
                .watch(&self.path, RecursiveMode::Recursive);
            if watcher.is_err() {
                //TODO handler err
                eprintln!("Couldn't start watcher");
                return false;
            } else {
                while !should_stop.load(std::sync::atomic::Ordering::Relaxed) {
                    sleep(Duration::from_millis(10));
                }
                return true;
            }
        } else {
            eprintln!("Couldn't setup watcher");
            return false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::OpenOptions;
    use std::io::Write;
    use std::sync::atomic::Ordering;
    use std::sync::mpsc::{channel, TryRecvError};
    use std::thread;
    use std::time::Duration;

    /// Tests the creation of a new `FileWatcher` instance.
    #[test]
    fn test_file_watcher_modify_file() {
        let path = PathBuf::from("examples/");
        let (tx, rx) = channel();
        let should_stop = Arc::new(AtomicBool::new(false));
        let watcher = FileWatcher::new(path.clone());

        let should_stop_clone = should_stop.clone();
        // Run the file watcher in a separate thread.
        let handle = thread::spawn(move || watcher.run(tx, should_stop_clone));

        thread::sleep(Duration::from_secs(1));

        // Open a file with append option
        let mut data_file = OpenOptions::new()
            .append(true)
            .open("examples/basics/c/basic-args.c")
            .expect("cannot open file");

        // Write to a file
        assert!(data_file.write_all("test".as_bytes()).is_ok());
        assert!(data_file.flush().is_ok());

        // Allow debounce time to trigger modification event.
        thread::sleep(Duration::from_secs(3));

        // Check if an event was received.
        match rx.try_recv() {
            Ok(event) => assert_eq!(event, Event::FileSaved),
            Err(TryRecvError::Empty) => panic!("Expected an event but got none"),
            Err(TryRecvError::Disconnected) => panic!("Channel disconnected"),
        }
        should_stop.store(true, Ordering::Relaxed);
        assert!(handle.join().expect("Couldn't join thread"));
    }
}
