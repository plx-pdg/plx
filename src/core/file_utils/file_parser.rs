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
    ExoSolutionNotFound(String),
    MultipleSolutionsFound(String),
}
