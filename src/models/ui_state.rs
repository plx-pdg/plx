use std::sync::Arc;

use super::{check_state::CheckState, exo::Exo, skill::Skill};

// The list of states and associated values for the UI to represent
#[derive(Debug, Clone, PartialEq)]
pub enum UiState {
    Home, // Home page with ASCII art
    Quit, // Exit in progress
    Help {
        scroll_offset: usize,
        last_state: Box<UiState>,
    }, // Help page with shortcuts documentation
    // List page
    SkillSelection {
        skill_index: usize,
        skills: Arc<Vec<Skill>>,
        exos: Arc<Vec<Exo>>,
    },
    ExoSelection {
        skill_index: usize,
        exo_index: usize,
        skills: Arc<Vec<Skill>>,
        exos: Arc<Vec<Exo>>,
    },
    ExoPreview {
        skill_index: usize,
        exo_index: usize,
        skills: Arc<Vec<Skill>>,
        exos: Arc<Vec<Exo>>,
        exo: Arc<Exo>,
    },
    // Train page in various steps
    Compiling {
        exo: Arc<Exo>,
    },
    CompileError {
        scroll_offset: usize,
        exo: Arc<Exo>,
        error: String,
    },
    CheckResults {
        scroll_offset: usize,
        exo: Arc<Exo>,
        checks: Vec<CheckState>,
    },
    ShowSolution {
        scroll_offset: usize,
        exo: Arc<Exo>,
        solution: String,
        solution_path: std::path::PathBuf,
        solution_idx: usize,
    },
}
