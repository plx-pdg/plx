use std::io;

use serde::{de::DeserializeOwned, Serialize};

use crate::core::file_utils::{
    file_parser::{ParseError, SerializeError},
    file_utils::{read_file, write_file},
};

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

pub fn write_object_to_file<T>(path: &std::path::PathBuf, object: T) -> Result<(), SerializeError>
where
    T: Serialize,
{
    let repr = toml_parser::toml_serialize(&object)
        .map_err(|err| SerializeError::SerializeError(err.to_string()))?;
    write_file(path, &repr).map_err(|err| SerializeError::WriteFileError(err.to_string()))?;
    Ok(())
}
