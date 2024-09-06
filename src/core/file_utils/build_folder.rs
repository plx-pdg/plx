use std::path::PathBuf;

use crate::models::constants::BUILD_FOLDER_NAME;
use crate::models::exo::Exo;

// Generates the exo build folder
// It will try to create a build/<skill>/<exo> folder at the root of the project
// Creates all necessary directories so if this function succeedes
// it means the directory is ready to be used
pub fn generate_build_folder(exo: &Exo) -> Result<std::path::PathBuf, std::io::Error> {
    let build_folder = if let Some(parent) = exo.folder.parent() {
        match (parent.file_name(), exo.folder.file_name()) {
            (Some(parent), Some(folder)) => {
                PathBuf::from(BUILD_FOLDER_NAME).join(parent).join(folder)
            }
            _ => std::env::temp_dir(),
        }
    } else {
        std::env::temp_dir()
    };
    if !build_folder.exists() {
        std::fs::create_dir_all(build_folder.clone())?;
    }

    Ok(build_folder)
}
