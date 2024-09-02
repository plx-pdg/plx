use serde::{Deserialize, Serialize};

use crate::core::file_utils::file_utils::read_file;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum ExoState {
    Todo,       // all checks are failing
    InProgress, // at least one successful check but not all of them
    Done,       // all checks are successful
}

impl ExoState {
    pub fn from_file(file_path: std::path::PathBuf) -> Self {
        if let Ok(file_content) = read_file(&file_path) {
            if file_content.contains("done") {
                ExoState::Done
            } else {
                ExoState::InProgress
            }
        } else {
            ExoState::Todo
        }
    }
}
