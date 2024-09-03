use crate::models::{check::Check, check_state::CheckState};

pub(super) struct ExoCheckResult {
    pub(super) state: CheckState,
    pub(super) output: Vec<String>,
}

impl ExoCheckResult {
    pub(super) fn new(check: &Check) -> Self {
        Self {
            state: CheckState::new(check),
            output: Vec::new(),
        }
    }
}
