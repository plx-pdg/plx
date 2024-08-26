use super::{check_state::CheckState, exo::Exo};

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
        exo: &'a Exo,
    },
    LoadingExo {
        exo: &'a Exo,
    },
    ExoLoaded {
        exo: &'a Exo,
    },
    Compiling {
        exo: &'a Exo,
    },
    CompileError {
        exo: &'a Exo,
        error: String,
    },
    DoingExo {
        exo: &'a Exo,
        checks: Vec<CheckState<'a>>,
    },
    ExoComplete {
        exo: &'a Exo,
    },
    ShowSolution {
        exo: &'a Exo,
    },
}
