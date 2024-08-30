use serde::Deserialize;

use crate::core::file_utils::file_parser::ParseError;

use super::skill::Skill;

#[derive(Deserialize)]
pub struct Project {
    name: String,
    skills: Vec<Skill>,
}

impl Project {
    pub fn from_dir(directory: std::path::PathBuf) -> Result<Self, ParseError> {
        let course_info_file = directory.push("course.toml");
        let skills_info_file = directory.push("skills.toml");
        let skills_folders = 
    }
}
