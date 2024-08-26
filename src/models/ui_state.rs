use super::{check_state::CheckState, exo::Exo};

// The list of states and associated values for the UI to represent
pub enum UiState<'a> {
    Home, // Home page with ASCII art
    Quit, // Exit in progress
    Help, // Help page with shortcuts documentation
    // List page
    SkillSelection {
        skill_index: usize,
    },
    ExoSelection {
        skill_index: usize,
        exo_index: usize,
    },
    ExoPreview {
        skill_index: usize,
        exo_index: usize,
        exo: &'a Exo,
    },
    // Train page in various steps
    Compiling {
        exo: &'a Exo,
    },
    CompileError {
        exo: &'a Exo,
        error: String,
    },
    CheckResults {
        exo: &'a Exo,
        checks: Vec<CheckState<'a>>,
    },
    ExoDone {
        exo: &'a Exo,
    },
    ShowSolution {
        exo: &'a Exo,
    },
}
