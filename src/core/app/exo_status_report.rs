use std::{path::PathBuf, sync::Arc};

use crate::models::{check_state::CheckState, exo::Exo};

use super::exo_check_result::ExoCheckResult;

pub(super) struct ExoStatusReport {
    pub(super) check_results: Vec<ExoCheckResult>,
    pub(super) compilation_output: Vec<String>,
    pub(super) elf_path: PathBuf,
    pub(super) exo: Arc<Exo>,
}

impl ExoStatusReport {
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

    pub(super) fn to_vec_check_state(&self) -> Vec<CheckState> {
        self.check_results
            .iter()
            .map(|result| result.state.clone())
            .collect()
    }
}
