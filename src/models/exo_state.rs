use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum ExoState {
    Todo,       // all checks are failing
    InProgress, // at least one successful check but not all of them
    Done,       // all checks are successful
}
impl Default for ExoState {
    fn default() -> Self {
        ExoState::Todo
    }
}
