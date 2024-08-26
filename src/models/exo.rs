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