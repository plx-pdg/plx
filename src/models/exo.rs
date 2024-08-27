use super::{check::Check, exo_state::ExoState, solution::Solution};

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
