use serde::{Deserialize, Serialize};
use super::{check::Check, exo_state::ExoState, solution::Solution};

#[derive(Serialize, Deserialize, Debug)]
struct ExoInfo {
    name: String,
    instruction: Option<String>,
    checks: Vec<Check>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ExoStateInfo {
    state: ExoState,
    favorite: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Exo {
    title: String,
    instruction: Option<String>,
    state: ExoState,
    files: Vec<std::path::PathBuf>,
    solution: Option<Solution>,
    checks: Vec<Check>,
    favorite: bool,
}
impl Exo {
    pub fn get_main_file(&self) -> Option<&std::path::PathBuf> {
        match self.files.iter().find(|file| {
            if let Some(file_name) = file.file_stem() {
                return file_name == "main";
            }
            return false;
        }) {
            Some(file) => Some(file),
            None => self.files.first(),
        }
    }
}
