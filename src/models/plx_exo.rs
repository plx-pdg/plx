use super::{plx_check::PlxCheck, plx_exo_state::PlxExoState, plx_solution::PlxSolution};

pub struct PlxExo {
    title: String,
    prompt: String,
    state: PlxExoState,
    files: Vec<std::path::PathBuf>,
    solution: Option<PlxSolution>,
    checks: Vec<PlxCheck>,
    favorite: bool,
}
