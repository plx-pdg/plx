use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Check {
    pub name: String,

    #[serde(default)]
    pub args: Vec<String>,

    pub test: CheckTest,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum CheckTest {
    #[serde(alias = "output")]
    Output { expected: String },
}
