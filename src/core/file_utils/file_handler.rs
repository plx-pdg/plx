use std::path::PathBuf;

pub fn is_plx_folder() -> bool {
    project_file().exists()
}
pub fn project_file() -> PathBuf {
    PathBuf::from(".plxproj")
}
