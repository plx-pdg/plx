use std::{fs::ReadDir, io};

pub fn read_file(file_path: &std::path::PathBuf) -> Result<String, io::Error> {
    std::fs::read_to_string(file_path)
}
pub fn list_dir(directory: &std::path::PathBuf) -> Result<ReadDir, io::Error> {
    std::fs::read_dir(directory)
}
