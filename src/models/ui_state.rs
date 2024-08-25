use super::{plx_check_state::PlxCheckState, plx_exo::PlxExo};

pub enum UiState<'a> {
    StartMenu,
    Quit,
    ChoosingSubject {
        subject_index: usize,
    },
    ChoosingExo {
        subject_index: usize,
        exo_index: usize,
    },
    ExoPromp {
        subject_index: usize,
        exo_index: usize,
        exo: &'a PlxExo,
    },
    LoadingExo {
        exo: &'a PlxExo,
    },
    ExoLoaded {
        exo: &'a PlxExo,
    },
    Compiling {
        exo: &'a PlxExo,
    },
    CompileError {
        exo: &'a PlxExo,
        error: String,
    },
    DoingExo {
        exo: &'a PlxExo,
        checks: Vec<PlxCheckState<'a>>,
    },
    ExoComplete {
        exo: &'a PlxExo,
    },
    ShowSolution {
        exo: &'a PlxExo,
    },
}
