pub enum UiState {
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
        exo: &PlxExo,
    },
    LoadingExo {
        exo: &PlxExo,
    },
    ExoLoaded {
        exo: &PlxExo,
    },
    Compiling {
        exo: &PlxExo,
    },
    CompileError {
        exo: &PlxExo,
        error: String,
    },
    DoingExo {
        exo: &PlxExo,
        checks: Vec<PlxCheckState>,
    },
    ExoComplete {
        exo: &PlxExo,
    },
    ShowSolution {
        exo: &PlxExo,
    },
}
