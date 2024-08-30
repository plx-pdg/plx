use serde::Deserialize;

use super::skill::Skill;

#[derive(Deserialize)]
pub struct Project {
    name: String,
    skills: Vec<Skill>,
}

impl TryFrom<std::path::PathBuf> for Project {
    type Error = file_parser::ParseError;

    fn try_from(_path: std::path::PathBuf) -> Result<Self, Self::Error> {
        // let content = read_file(path)?;
        // parse_yaml(content)?;
        Err(file_parser::ParseError::ParseError)
    }
}
