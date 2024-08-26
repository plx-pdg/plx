use super::exo::Exo;

pub struct Subject {
    name: String,
    path: std::path::PathBuf,
    exos: Vec<Exo>,
}
