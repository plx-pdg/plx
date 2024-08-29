use std::path::PathBuf;
use std::sync::mpsc:: Sender;
use std::time::Duration;
use notify::{RecursiveMode, Watcher};
use notify_debouncer_mini::{notify::*, new_debouncer, DebounceEventResult, DebouncedEventKind};

use crate::{models::{event::Event, exo::Exo}};

/// Represents a file watcher that monitors changes to files in a specified directory.
///
/// The `FileWatcher` struct allows monitoring of a specific path for changes to files
/// with particular extensions and triggers events when those changes are detected.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FileWatcher{
    exo: Exo,
}

impl FileWatcher {
    /// Creates a new `FileWatcher` instance.
    ///
    /// # Arguments
    ///
    /// * `exo` - An instance of the Exo struct.
    ///
    /// # Returns
    ///
    /// A new `FileWatcher` instance.
    pub fn new(exo: Exo) -> Self{
        FileWatcher{exo}
    }
    /// Starts monitoring the specified path for changes.
    ///
    /// This function sets up a debouncer that watches for changes in files under the given path
    /// and sends an event if a change is detected for an allowed file type.
    ///
    /// # Arguments
    ///
    /// * `tx` - A sender channel for sending events when changes are detected.
    pub fn run(&self, tx: Sender<Event>){
        let allowed_extensions = self.exo.allowed_extensions.clone();
        let mut debouncer = new_debouncer(Duration::from_secs(2),move |res: DebounceEventResult| {
            match res {
                Ok(events) => {
                    for event in events {
                        // Check if the file type is allowed
                        if FileWatcher::is_allowed_file_type(&allowed_extensions, &event.path) {
                            if let DebouncedEventKind::Any = event.kind {
                                tx.send(Event::FileSaved).unwrap();
                            }
                        }
                    }
                }
                Err(_) => {
                    tx.send(Event::NoFileSaved).unwrap();
                }
            }
        }).unwrap();

        debouncer.watcher().watch(&self.exo.dir_path, RecursiveMode::Recursive).unwrap();

    }

    /// Checks if the given file path has an allowed file extension.
    ///
    /// # Arguments
    ///
    /// * `allowed_extensions` - A list of allowed file extensions.
    /// * `path` - The path of the file to check.
    ///
    /// # Returns
    ///
    /// `true` if the file has an allowed extension, otherwise `false`.
    fn is_allowed_file_type(allowed_extensions: &Vec<String>, path: &PathBuf) -> bool{
        if let Some(extension) = path.extension(){
            if let Some(ext_str) = extension.to_str() {
                return allowed_extensions.contains(&ext_str.to_lowercase());
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc::{channel, TryRecvError};
    use std::thread;
    use std::fs::{File, remove_file};
    use std::time::Duration;
    use tempfile::tempdir;
    use crate::models::exo_state::ExoState;

    /// Tests the creation of a new `FileWatcher` instance.
    #[test]
    fn test_file_watcher_creation() {
        let exo = Exo::new(
            "test".to_string(),
            None,
            ExoState::Todo,
            PathBuf::from("examples"),
            vec![PathBuf::from("examples/basics/c/basic-args.c"),
                 PathBuf::from("examples/README.md"),
                 PathBuf::from("examples/basics/cpp/basic-args.cpp")],
            None,
            vec!["cpp".to_string(), "c".to_string()],
            None,
            false
        );
        let watcher = FileWatcher::new(exo.clone());

        assert_eq!(watcher.exo, exo);
    }

    /// Tests the `is_allowed_file_type` method for various file extensions.
    #[test]
    fn test_is_allowed_file_type() {
        let allowed_extensions = vec!["cpp".to_string(), "c".to_string()];

        let path_txt = PathBuf::from("examples/basics/cpp/basic-args.cpp");
        let path_rs = PathBuf::from("examples/basics/cpp/basic-args.cpp");
        let path_other = PathBuf::from("examples/README.md");

        assert!(FileWatcher::is_allowed_file_type(&allowed_extensions, &path_txt));
        assert!(FileWatcher::is_allowed_file_type(&allowed_extensions, &path_rs));
        assert!(!FileWatcher::is_allowed_file_type(&allowed_extensions, &path_other));
    }

    // Tests the `run` method to ensure it sends the correct events for allowed file types.
    // #[test]
    // fn test_run() {
    //     let exo = Exo::new(
    //         "test".to_string(),
    //         None,
    //         ExoState::Todo,
    //         PathBuf::from("src/core/watcher/test/"),
    //         vec![PathBuf::from("src/core/watcher/test/file.md"),
    //              PathBuf::from("src/core/watcher/test/file.rs"), PathBuf::from("src/core/watcher/test/file.txt")],
    //         None,
    //         vec!["txt".to_string(), "rs".to_string()],
    //         None,
    //         false
    //     );
    //
    //      let watcher = FileWatcher::new(exo);
    //
    //      let (tx, rx) = channel();
    //
    //      // Run the watcher in a separate thread to avoid blocking the test
    //      thread::spawn(move || {
    //          watcher.run(tx);
    //      });
    //
    //      // Create a temporary file with an allowed extension
    //      let file_path = temp_path.join("test.txt");
    //      File::create(&file_path).unwrap();
    //
    //     // Wait for a short duration to allow the watcher to detect changes
    //      thread::sleep(Duration::from_secs(3));
    //
    //      // Check if the event was received
    //      match rx.try_recv() {
    //          Ok(Event::FileSaved) => println!("FileSaved event detected"),
    //          Ok(_) => panic!("Unexpected event detected"),
    //          Err(TryRecvError::Empty) => panic!("No event detected"),
    //          Err(TryRecvError::Disconnected) => panic!("Channel disconnected"),
    //      }
    //
    //      // Cleanup the temporary file
    //      remove_file(file_path).unwrap();
    //  }
}
