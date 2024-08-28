use crate::core::file_utils::file_parser;

use super::skill::Skill;

pub struct Project {
    name: String,
    skills: Vec<Skill>,
}

//Template code for testing ui. Should not be merged into main !!!!
impl Project {
    pub fn new() -> Self {
        Project {
            name: String::from("Project"),
            skills: vec![],
        }
    }
}
impl TryFrom<std::path::PathBuf> for Project {
    type Error = file_parser::ParseError;

    fn try_from(_path: std::path::PathBuf) -> Result<Self, Self::Error> {
        // let content = read_file(path)?;
        // parse_yaml(content)?;
        Err(file_parser::ParseError::ParseError)
    }
}
