use std::{
    path,
    sync::{atomic::AtomicBool, mpsc::Sender, Arc},
    thread::{self},
};

use crate::{
    core::process::process_handler::{self, wait_child},
    models::event::Event,
};

pub fn run(
    tx: Sender<Event>,
    editor: String,
    file_path: path::PathBuf,
    should_stop: Arc<AtomicBool>,
) {
    Some(thread::spawn(move || loop {
        let child = process_handler::spawn_process(&editor, vec![file_path.display().to_string()]);
        let _ = match child {
            Ok(mut child) => match wait_child(&mut child, should_stop.clone()) {
                Ok(_) => tx.send(Event::EditorOpened),
                Err(_) => tx.send(Event::CouldNotOpenEditor),
            },
            Err(_) => tx.send(Event::CouldNotOpenEditor),
        };
    }));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{str::FromStr, sync::mpsc};

    #[test]
    fn opens_file() {
        let (tx, rx) = mpsc::channel();
        let should_stop = Arc::new(AtomicBool::new(false));
        run(
            tx,
            "echo".to_string(),
            path::PathBuf::from_str("./opener.rs").unwrap(),
            should_stop.clone(),
        );
        assert_eq!(rx.recv().unwrap(), Event::EditorOpened);
    }

    #[test]
    fn opens_file_missing_editor() {
        let (tx, rx) = mpsc::channel();
        let should_stop = Arc::new(AtomicBool::new(false));
        run(
            tx,
            "_".to_string(),
            path::PathBuf::from_str("./opener.rs").unwrap(),
            should_stop.clone(),
        );
        assert_eq!(rx.recv().unwrap(), Event::CouldNotOpenEditor);
    }
}
