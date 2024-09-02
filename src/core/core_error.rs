use core::fmt;

pub enum CoreInitError {
    PlxProjNotFound,
    ProjFilesParsingError(String),
}

impl fmt::Display for CoreInitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CoreInitError::PlxProjNotFound => write!(f, "PLX didn't find a course.toml in the current folder, this is not the root of a PLX course."),
            CoreInitError::ProjFilesParsingError(detail) => {
                write!(f, "Parsing error: {}", detail)
            }
        }
    }
}
