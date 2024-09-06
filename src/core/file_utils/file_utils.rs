use std::{fs::ReadDir, io};

pub fn read_file(file_path: &std::path::PathBuf) -> Result<String, io::Error> {
    std::fs::read_to_string(file_path)
}

pub fn write_file(file_path: &std::path::PathBuf, content: &str) -> Result<(), io::Error> {
    std::fs::write(file_path, content)
}
pub fn list_dir(directory: &std::path::PathBuf) -> Result<ReadDir, io::Error> {
    std::fs::read_dir(directory)
}

// From https://stackoverflow.com/a/58063083
pub fn list_dir_folders(dir: &std::path::PathBuf) -> Result<Vec<std::path::PathBuf>, io::Error> {
    Ok(std::fs::read_dir(dir)?
        .into_iter()
        .filter(|r| r.is_ok()) // Get rid of Err variants for Result<DirEntry>
        .map(|r| r.unwrap().path()) // This is safe, since we only have the Ok variants
        .filter(|r| r.is_dir()) // Filter out non-folders
        .collect())
}

// From https://stackoverflow.com/a/58063083
pub fn list_dir_files(dir: &std::path::PathBuf) -> Result<Vec<std::path::PathBuf>, io::Error> {
    Ok(std::fs::read_dir(dir)?
        .into_iter()
        .filter(|r| r.is_ok()) // Get rid of Err variants for Result<DirEntry>
        .map(|r| r.unwrap().path()) // This is safe, since we only have the Ok variants
        .filter(|r| r.is_file()) // Filter out folders
        .collect())
}

// From https://stackoverflow.com/a/38384901
pub fn get_full_path(path: &std::path::PathBuf) -> Result<std::path::PathBuf, io::Error> {
    dunce::canonicalize(path)
}

pub fn current_folder() -> Result<std::path::PathBuf, io::Error> {
    std::env::current_dir()
}
