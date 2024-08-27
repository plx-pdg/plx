use std::{
    path,
    sync::{atomic::AtomicBool, mpsc::Sender, Arc},
};

use crate::{
    core::process::process_handler::{self, wait_child},
    models::event::Event,
};

use super::editor::get_default_editor;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EditorOpener {
    editor: String,
    file_path: path::PathBuf,
}

impl EditorOpener {
    pub fn new(editor: String, file_path: path::PathBuf) -> Self {
        EditorOpener { editor, file_path }
    }

    pub fn new_default_editor(file_path: path::PathBuf) -> Option<Self> {
        match get_default_editor() {
            Some(editor) => Some(EditorOpener::new(editor, file_path)),
            None => None,
        }
    }
}
impl EditorOpener {
    pub fn run(&self, tx: Sender<Event>, should_stop: Arc<AtomicBool>) {
        let child = process_handler::spawn_process(
            &self.editor,
            vec![self.file_path.display().to_string()],
        );
        let _ = match child {
            Ok(mut child) => match wait_child(&mut child, should_stop.clone()) {
                Ok(_) => tx.send(Event::EditorOpened),
                Err(_) => tx.send(Event::CouldNotOpenEditor),
            },
            Err(_) => tx.send(Event::CouldNotOpenEditor),
        };
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
        let worker = EditorOpener::new("echo".to_string(), ".opener.rs".into());
        worker.run(tx.clone(), should_stop.clone());
        assert_eq!(rx.recv().unwrap(), Event::EditorOpened);
    }

    #[test]
    fn opens_file_missing_editor() {
        let (tx, rx) = mpsc::channel();
        let should_stop = Arc::new(AtomicBool::new(false));
        let worker = EditorOpener::new("_".to_string(), ".opener.rs".into());
        worker.run(tx.clone(), should_stop.clone());
        assert_eq!(rx.recv().unwrap(), Event::CouldNotOpenEditor);
    }
}
