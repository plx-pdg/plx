use std::sync::Arc;

use super::check::Check;

#[derive(Debug, Clone)]
pub struct CheckState {
    pub(crate) check: Arc<Check>,
    pub(crate) passed: bool,
}
