use crate::core::file_utils::file_parser::{ParseError, ParseWarning};

/// Trait to standardize the creation using a directory folder
/// Basically every level of a a plx project can be created using a directory path.
/// See `models::Project`, `models::Skill` and `models::Exo`
pub trait FromDir {
    fn from_dir(
        dir: &std::path::PathBuf,
    ) -> Result<(Self, Vec<ParseWarning>), (ParseError, Vec<ParseWarning>)>
    where
        Self: Sized;
}
