use notify::RecursiveMode;
use notify_debouncer_mini::{new_debouncer, DebounceEventResult, DebouncedEventKind};
use std::path::PathBuf;
use std::sync::{atomic::AtomicBool, mpsc::Sender, Arc};
use std::thread::sleep;
use std::time::Duration;

use crate::models::event::Event;

#[derive(Clone, Debug, PartialEq, Eq)]
/// A structure representing a file or directory watcher.
pub struct FileWatcher {
    /// The path to the file or directory to be watched.
    path: PathBuf,
}

impl FileWatcher {
    /// Creates a new instance of `FileWatcher` with the specified path.
    ///
    /// # Arguments
    ///
    /// * `path` - A `PathBuf` specifying the path of the file or directory to watch.
    ///
    /// # Returns
    ///
    /// A new instance of `FileWatcher`
    pub fn new(path: PathBuf) -> Self {
        FileWatcher { path }
    }

    /// Runs a file or directory watcher.
    ///
    /// This function blocks and should be called from a separate thread. It watches for changes in
    /// the specified path and sends an `Event::FileSaved` message on the provided channel when a change is detected.
    ///
    /// # Arguments
    ///
    /// * `tx` - A sender channel used to send events when changes are detected.
    /// * `should_stop` - An atomic boolean flag used to signal when the watcher should stop running.
    ///
    /// # Returns
    ///
    /// Returns `true` if the watcher was initialized and ran successfully, otherwise returns `false`.
    pub fn run(self, tx: Sender<Event>, should_stop: Arc<AtomicBool>) -> bool {
        // Create a new debouncer with a delay of 1 second.
        let debouncer =
            new_debouncer(
                Duration::from_secs(1),
                move |res: DebounceEventResult| match res {
                    // Handle debounced events
                    Ok(events) => {
                        for event in events {
                            if let DebouncedEventKind::Any = event.kind {
                                // Send an event if a file change is detected
                                if tx.send(Event::FileSaved).is_err() {
                                    return;
                                }
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
                false
            } else {
                while !should_stop.load(std::sync::atomic::Ordering::Relaxed) {
                    sleep(Duration::from_millis(10));
                }
                true
            }
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::OpenOptions;
    use std::io::Write;
    use std::sync::atomic::Ordering;
    use std::sync::mpsc::{channel, RecvTimeoutError};
    use std::thread;
    use std::time::Duration;

    /// Tests the `FileWatcher` functionality for modifications in a nested folder within a directory.
    #[test]
    fn test_file_watcher_modify_with_recursive_inside_tree_folder() {
        let path = PathBuf::from("examples");
        let (tx, rx) = channel();
        let should_stop = Arc::new(AtomicBool::new(false));
        let watcher = FileWatcher::new(path.clone());

        let should_stop_clone = should_stop.clone();
        // Run the file watcher in a separate thread.
        let handle = thread::spawn(move || watcher.run(tx, should_stop_clone));

        sleep(Duration::from_secs(1));
        {
            // Open a file with append option
            let mut data_file = OpenOptions::new()
                .append(true)
                .open(path.join("basics").join("c").join("basic-args.c"))
                .expect("cannot open file");

            // Write to a file
            assert!(data_file.write_all("test".as_bytes()).is_ok());
            assert!(data_file.flush().is_ok());
        }
        // Check if an event was received.
        match rx.recv_timeout(Duration::from_secs(5)) {
            Ok(event) => assert_eq!(event, Event::FileSaved),
            Err(RecvTimeoutError::Timeout) => panic!("Timed out waiting for event"),
            Err(RecvTimeoutError::Disconnected) => panic!("Channel disconnected"),
        }
        should_stop.store(true, Ordering::Relaxed);
        assert!(handle.join().expect("Couldn't join thread"));
    }

    /// Tests the `FileWatcher` functionality for modifications in the root of the watched directory.
    #[test]
    fn test_file_watcher_modify_with_recursive_racine_folder() {
        let path = PathBuf::from("examples");
        let (tx, rx) = channel();
        let should_stop = Arc::new(AtomicBool::new(false));
        let watcher = FileWatcher::new(path.clone());

        let should_stop_clone = should_stop.clone();
        // Run the file watcher in a separate thread.
        let handle = thread::spawn(move || watcher.run(tx, should_stop_clone));

        sleep(Duration::from_secs(1));
        {
            // Open a file with append option
            let mut data_file = OpenOptions::new()
                .append(true)
                .open(path.join("README.md"))
                .expect("cannot open file");

            // Write to a file
            assert!(data_file.write_all("test".as_bytes()).is_ok());
            assert!(data_file.flush().is_ok());
        }
        // Check if an event was received.
        match rx.recv_timeout(Duration::from_secs(5)) {
            Ok(event) => assert_eq!(event, Event::FileSaved),
            Err(RecvTimeoutError::Timeout) => panic!("Timed out waiting for event"),
            Err(RecvTimeoutError::Disconnected) => panic!("Channel disconnected"),
        }
        should_stop.store(true, Ordering::Relaxed);
        assert!(handle.join().expect("Couldn't join thread"));
    }

    /// Tests the `FileWatcher` functionality for modifications of a single file without recursive mode.
    #[test]
    fn test_file_watcher_modify_without_recursive() {
        let path = PathBuf::from("examples/basics/c/basic-args.c");
        let (tx, rx) = channel();
        let should_stop = Arc::new(AtomicBool::new(false));
        let watcher = FileWatcher::new(path.clone());

        let should_stop_clone = should_stop.clone();
        // Run the file watcher in a separate thread.
        let handle = thread::spawn(move || watcher.run(tx, should_stop_clone));

        sleep(Duration::from_secs(1));
        {
            // Open a file with append option
            let mut data_file = OpenOptions::new()
                .append(true)
                .open(path)
                .expect("cannot open file");

            // Write to a file
            assert!(data_file.write_all("test".as_bytes()).is_ok());
            assert!(data_file.flush().is_ok());
        }
        // Check if an event was received.
        match rx.recv_timeout(Duration::from_secs(5)) {
            Ok(event) => assert_eq!(event, Event::FileSaved),
            Err(RecvTimeoutError::Timeout) => panic!("Timed out waiting for event"),
            Err(RecvTimeoutError::Disconnected) => panic!("Channel disconnected"),
        }
        should_stop.store(true, Ordering::Relaxed);
        assert!(handle.join().expect("Couldn't join thread"));
    }
}
