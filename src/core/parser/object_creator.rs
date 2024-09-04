use serde::de::DeserializeOwned;

use crate::core::file_utils::{file_parser::ParseError, file_utils::read_file};

use super::toml_parser;

pub fn create_from_file<T>(path: &std::path::PathBuf) -> Result<T, ParseError>
where
    T: DeserializeOwned,
{
    let file_content = read_file(path).map_err(|err| ParseError::ReadFileError(err.to_string()))?;
    let me = toml_parser::read_from_toml(&file_content)
        .map_err(|err| ParseError::ParseError(err.to_string()))?;
    Ok(me)
}
