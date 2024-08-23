use super::plx_exo::PlxExo;

pub struct PlxSubject {
    name: String,
    path: std::path::PathBuf,
    exos: Vec<PlxExo>,
}
