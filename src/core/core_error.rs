use core::fmt;

pub enum CoreInitError {
    PlxProjNotFound,
    ProjFilesParsingError(String),
}

impl fmt::Display for CoreInitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CoreInitError::PlxProjNotFound => write!(f, ".plxproj folder not found"),
            CoreInitError::ProjFilesParsingError(detail) => {
                write!(f, "Parsing error {}", detail)
            }
        }
    }
}
