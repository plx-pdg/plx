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
    path: PathBuf,
    exo: Exo,
    allowed_extensions: Vec<String>,
}

impl FileWatcher {
    /// Creates a new `FileWatcher` instance.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to be watched.
    /// * `exo` - An instance of the Exo struct.
    /// * `allowed_extensions` - A list of file extensions that should be monitored.
    ///
    /// # Returns
    ///
    /// A new `FileWatcher` instance.
    pub fn new(path: PathBuf, exo: Exo, allowed_extensions: Vec<String>) -> Self{
        FileWatcher{path, exo, allowed_extensions}
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
        let allowed_extensions = self.allowed_extensions.clone();
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
                Err(e) => {
                    tx.send(Event:: NoFileSaved).unwrap();
                }
            }
        }).unwrap();

        debouncer.watcher().watch(&self.path, RecursiveMode::Recursive).unwrap();
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

    #[test]
    fn modify_and_save_file() {

    }

    #[test]
    fn no_modif_file() {

    }
}
