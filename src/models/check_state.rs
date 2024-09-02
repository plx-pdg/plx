use std::sync::Arc;

use super::check::Check;

pub struct CheckState {
    check: Arc<Check>,
    passed: bool,
}
