use std::fs::ReadDir;
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
    name: String,
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
impl Exo {
    fn find_exo_files(
        dir_entries: ReadDir,
    ) -> (Vec<std::path::PathBuf>, Option<std::path::PathBuf>) {
        let mut exo_files = Vec::new();
        let mut solution_file = None;
        for entry in dir_entries {
            if let Ok(entry) = entry {
                let file_path = entry.path();
                let file_path_str = file_path.display().to_string();
                let file_extension = file_path
                    .extension()
                    .and_then(|extension| extension.to_str())
                    .or(Some(""))
                    .unwrap();

                if file_path_str.contains(".sol.") {
                    // TODO handle the case where we have multiple solutions ?
                    solution_file = Some(file_path);
                    continue;
                }
                if file_extension == "toml" {
                    continue;
                }
                // TODO maybe make sure we don't mix .c with .java files here ?
                // We need to be careful adding this because .c can be mixed with .cpp, .h,
                // .hpp etc...
                exo_files.push(file_path);
            }
        }
        (exo_files, solution_file)
    }
}
