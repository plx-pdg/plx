use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Solution {
    pub path: std::path::PathBuf,
}
impl Solution {
    pub fn new(path: std::path::PathBuf) -> Self {
        Solution { path }
    }
}
