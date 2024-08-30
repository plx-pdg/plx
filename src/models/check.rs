use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Check {
    pub name: String,

    #[serde(default)]
    pub args: Vec<String>,

    #[serde(alias = "type")]
    pub check_type: CheckType,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum CheckType {
    #[serde(alias = "output")]
    Output,
}
