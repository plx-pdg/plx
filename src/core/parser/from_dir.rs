use crate::core::file_utils::file_parser::{ParseError, ParseWarning};

pub trait FromDir {
    fn from_dir(
        dir: &std::path::PathBuf,
    ) -> Result<(Self, Vec<ParseWarning>), (ParseError, Vec<ParseWarning>)>
    where
        Self: Sized;
}
