use std::{path::PathBuf, sync::Arc};

use crate::models::{check_state::CheckState, exo::Exo};

use super::exo_check_result::ExoCheckResult;

/// ExoStatusReport
///
/// This struct is used to store the result of a run + check
/// It keeps the information of an exo run, including the check results,
/// the compilation output and the path to the elf file
/// See `ExoCheckResult` for more information about the check results
pub(super) struct ExoStatusReport {
    pub(super) check_results: Vec<ExoCheckResult>,
    pub(super) compilation_output: Vec<String>,
    pub(super) elf_path: PathBuf,
    pub(super) exo: Arc<Exo>,
}

impl ExoStatusReport {
    /// Create an ExoStatusReport from an Exo and the target path
    pub(super) fn new(exo: &Exo, elf_path: PathBuf) -> Self {
        let checkers: Vec<ExoCheckResult> = exo
            .checks
            .iter()
            .map(|check| ExoCheckResult::new(check))
            .collect();

        Self {
            check_results: checkers,
            compilation_output: Vec::new(),
            elf_path,
            exo: Arc::new(exo.clone()),
        }
    }

    /// Helper function to get a `Vec<CheckState>` from check results
    /// Useful to send the check states to the Ui
    /// Check `UiState::CheckResults` for more information
    pub(super) fn to_vec_check_state(&self) -> Vec<CheckState> {
        self.check_results
            .iter()
            .map(|result| result.state.clone())
            .collect()
    }
}
