use std::{io, path::PathBuf};

pub fn is_plx_folder() -> bool {
    project_file().exists()
}
pub fn current_folder() -> Result<PathBuf, io::Error> {
    std::env::current_dir()
}
pub fn project_file() -> PathBuf {
    PathBuf::from(".plxproj")
}
