use serde::{Deserialize, Serialize};

/// Represents a Exo Check
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Check {
    pub name: String,

    #[serde(default)]
    pub args: Vec<String>,

    pub test: CheckTest,
}

/// Represents the actual check type
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(tag = "type")]
pub enum CheckTest {
    #[serde(alias = "output")]
    Output { expected: String },
}
