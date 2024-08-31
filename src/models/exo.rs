use std::path::PathBuf;
use super::{check::Check, exo_state::ExoState, solution::Solution};
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Exo {
    title: String,
    instruction: Option<String>,
    state: ExoState,
    files: Vec<PathBuf>,
    solution: Option<Solution>,
    checks: Option<Vec<Check>>,
    favorite: bool
}
impl Exo {
    pub fn new(title: String,
               instruction: Option<String>,
               state: ExoState,
               dir_path: PathBuf,
               files: Vec<PathBuf>,
               solution: Option<Solution>,
               allowed_extensions: Vec<String>,
               checks: Option<Vec<Check>>,
               favorite: bool) -> Self {
        Exo{title, instruction, state, files, solution, checks, favorite}
    }
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
