use serde::Deserialize;
use super::exo::Exo;

#[derive(Deserialize)]
pub struct Skill {
    name: String,
    path: std::path::PathBuf,
    exos: Vec<Exo>,
}
