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
