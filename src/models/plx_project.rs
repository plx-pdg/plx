use crate::core::file_utils::file_parser;

use super::plx_subject::PlxSubject;

pub struct PlxProject {
    name: String,
    subjects: Vec<PlxSubject>,
}
impl PlxProject {
    pub fn new(name: String) -> Self {
        PlxProject {
            name,
            subjects: Vec::new(),
        }
    }
}

impl TryFrom<std::path::PathBuf> for PlxProject {
    type Error = file_parser::ParseError;

    fn try_from(_path: std::path::PathBuf) -> Result<Self, Self::Error> {
        // let content = read_file(path)?;
        // parse_yaml(content)?;
        Ok(PlxProject {
            name: "New Project".to_string(),
            subjects: todo!(),
        })
    }
}
