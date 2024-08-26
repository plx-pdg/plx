use super::exo::Exo;

pub struct Skill {
    name: String,
    path: std::path::PathBuf,
    exos: Vec<Exo>,
}
