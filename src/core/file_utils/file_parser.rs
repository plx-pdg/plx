use core::fmt;

#[derive(Debug)]
pub enum ParseError {
    ReadFileError(String),
    ParseError(String),
    FileNotFound(String),
    FileDiscoveryFailed(String),
    NoExoFilesFound(std::path::PathBuf),
    ErrorParsingSkills(String),
    ErrorParsingExos(String),
}
#[derive(Debug, PartialEq, Eq)]
pub enum ParseWarning {
    ParseSkillFail(String),
    ParseExoFail(String),
    NoSolutionFile(String),
    ExoFileNotFound(String),
    MultipleSolutionsFound(String),
    InvalidFileName(String),
}

#[derive(Debug)]
pub enum SerializeError {
    SerializeError(String),
    WriteFileError(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::ReadFileError(detail) => write!(f, "Error Reading File ({detail})"),
            ParseError::ParseError(detail) => write!(f, "Error Parsing File ({detail})"),
            ParseError::FileNotFound(detail) => write!(f, "File Not Found ({detail})"),
            ParseError::FileDiscoveryFailed(detail) => {
                write!(f, "File Discovery Failed ({detail})")
            }
            ParseError::NoExoFilesFound(folder) => {
                write!(f, "No Exo Files Found {}", folder.to_str().unwrap_or(""))
            }
            ParseError::ErrorParsingSkills(detail) => write!(f, "Error Parsing Skills ({detail})"),
            ParseError::ErrorParsingExos(detail) => write!(f, "Error Parsing Exos ({detail})"),
        }
    }
}
impl fmt::Display for ParseWarning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseWarning::ParseSkillFail(detail) => write!(f, "Parse Skill Fail ({})", detail),
            ParseWarning::ParseExoFail(detail) => write!(f, "Parse Exo Fail ({})", detail),
            ParseWarning::NoSolutionFile(detail) => write!(f, "No Solution File ({})", detail),
            ParseWarning::ExoFileNotFound(detail) => write!(f, "Exo File Not Found ({})", detail),
            ParseWarning::MultipleSolutionsFound(detail) => {
                write!(f, "Multiple Solutions Found ({})", detail)
            }
            ParseWarning::InvalidFileName(detail) => write!(f, "Invalid File Name ({})", detail),
        }
    }
}
impl fmt::Display for SerializeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SerializeError::SerializeError(detail) => write!(f, "Serialize Error {detail}"),
            SerializeError::WriteFileError(detail) => write!(f, "Write File Error {detail}"),
        }
    }
}
