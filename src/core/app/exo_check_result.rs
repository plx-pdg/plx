use crate::models::{check::Check, check_state::CheckState};

///
/// ExoCheckResult
///
/// This struct is used to store the result of a run + check
/// Each exo run will have as many ExoCheckResults as the number of checks the exo has
/// This helps us keep the output of the run and the check state together
///
pub(super) struct ExoCheckResult {
    pub(super) state: CheckState,
    pub(super) output: Vec<String>,
}

impl ExoCheckResult {
    /// Create an ExoCheckResult from a Check
    pub(super) fn new(check: &Check) -> Self {
        Self {
            state: CheckState::new(check),
            output: Vec::new(),
        }
    }
}
