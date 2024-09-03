use super::{check_state::CheckState, exo::Exo, skill::Skill};
use std::sync::Arc;

// The list of states and associated values for the UI to represent
pub enum UiState {
    Home, // Home page with ASCII art
    Quit, // Exit in progress
    Help, // Help page with shortcuts documentation
    // List page
    SkillSelection {
        skills: Arc<Vec<Skill>>,
        skill_index: usize,
    },
    ExoSelection {
        skills: Arc<Vec<Skill>>,
        skill_index: usize,
        exos: Arc<Vec<Exo>>,
        exo_index: usize,
    },
    ExoPreview {
        skill_index: usize,
        exo_index: usize,
        exo: Arc<Exo>,
    },
    // Train page in various steps
    Compiling {
        exo: Arc<Exo>,
    },
    CompileError {
        exo: Arc<Exo>,
        error: String,
    },
    CheckResults {
        exo: Arc<Exo>,
        checks: Vec<CheckState>,
    },
    ExoDone {
        exo: Arc<Exo>,
    },
    ShowSolution {
        exo: Arc<Exo>,
    },
}
