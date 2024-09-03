use std::path::PathBuf;

use crate::models::exo::Exo;

pub fn generate_build_folder(exo: &Exo) -> Result<std::path::PathBuf, std::io::Error> {
    let build_folder = if let Some(folder_name) = exo.folder.file_name() {
        PathBuf::from("build").join(folder_name)
    } else {
        std::env::temp_dir()
    };
    if !build_folder.exists() {
        std::fs::create_dir_all(build_folder.clone())?;
    }
    Ok(build_folder)
}
