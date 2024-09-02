use std::sync::Arc;

use super::check::Check;

#[derive(Debug, Clone)]
pub struct CheckState {
    check: Arc<Check>,
    passed: bool,
}
