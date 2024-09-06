use std::{
    path,
    sync::{
        atomic::AtomicBool,
        mpsc::{channel, Sender},
        Arc,
    },
};

use crate::{
    core::{
        runner::runner::{RunEvent, Runner},
        work::{work::Work, work_type::WorkType},
    },
    models::event::Event,
};

use super::editor::get_default_editor;

// Editor Opener Worker
pub struct EditorOpener {
    runner: Runner,
}

impl EditorOpener {
    // Tries to create self using a given editor
    pub fn new(editor: String, file_path: path::PathBuf) -> Option<Self> {
        if let Some(file_path) = file_path.to_str() {
            return Some(EditorOpener {
                runner: Runner::new(editor, vec![file_path.to_string()]),
            });
        }
        None
    }

    // Tries to create self using the default editor
    // See `editor::get_default_editor` for more info
    pub fn new_default_editor(file_path: path::PathBuf) -> Option<Self> {
        match get_default_editor() {
            Some(editor) => Some(EditorOpener::new(editor, file_path)?),
            None => None,
        }
    }
}
impl Work for EditorOpener {
    fn work_type(&self) -> WorkType {
        WorkType::EditorOpen
    }
    // Launches a runner to open the chosen editor
    // /!\ terminal based editors are not very well supported at this point in time /!\
    fn run(&self, tx: Sender<Event>, should_stop: Arc<AtomicBool>) -> bool {
        let (runner_tx, rx) = channel();

        let _ = self.runner.run(runner_tx, should_stop);
        while let Ok(msg) = rx.recv() {
            match msg {
                RunEvent::ProcessCreationFailed(_err) => {
                    let _ = tx.send(Event::CouldNotOpenEditor);
                    return false;
                }

                // For now, if the process was created, just assume we were able to open the editor
                RunEvent::ProcessCreated => {
                    let _ = tx.send(Event::EditorOpened);
                    return true;
                }
                _ => {}
            }
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;

    #[test]
    fn opens_file() {
        let (tx, rx) = mpsc::channel();
        let should_stop = Arc::new(AtomicBool::new(false));
        // Use "echo" as an "editor" so this test can be passed in ci
        let worker = EditorOpener::new("echo".to_string(), ".opener.rs".into()).unwrap();
        worker.run(tx.clone(), should_stop.clone());
        assert_eq!(rx.recv().unwrap(), Event::EditorOpened);
    }

    #[test]
    fn opens_file_missing_editor() {
        let (tx, rx) = mpsc::channel();
        let should_stop = Arc::new(AtomicBool::new(false));
        let worker = EditorOpener::new("_".to_string(), ".opener.rs".into()).unwrap();
        worker.run(tx.clone(), should_stop.clone());
        assert_eq!(rx.recv().unwrap(), Event::CouldNotOpenEditor);
    }
}
