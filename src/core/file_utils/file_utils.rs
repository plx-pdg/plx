use std::{fs::ReadDir, io};

pub fn read_file(file_path: &std::path::PathBuf) -> Result<String, io::Error> {
    std::fs::read_to_string(file_path)
}
pub fn list_dir(directory: &std::path::PathBuf) -> Result<ReadDir, io::Error> {
    std::fs::read_dir(directory)
}

pub fn list_dir_folders(dir: &std::path::PathBuf) -> Result<Vec<std::path::PathBuf>, io::Error> {
    Ok(std::fs::read_dir(dir)?
        .into_iter()
        .filter(|r| r.is_ok()) // Get rid of Err variants for Result<DirEntry>
        .map(|r| r.unwrap().path()) // This is safe, since we only have the Ok variants
        .filter(|r| r.is_dir()) // Filter out non-folders
        .collect())
}
pub fn list_dir_files(dir: &std::path::PathBuf) -> Result<Vec<std::path::PathBuf>, io::Error> {
    Ok(std::fs::read_dir(dir)?
        .into_iter()
        .filter(|r| r.is_ok()) // Get rid of Err variants for Result<DirEntry>
        .map(|r| r.unwrap().path()) // This is safe, since we only have the Ok variants
        .filter(|r| r.is_file()) // Filter out folders
        .collect())
}
