use crate::core::file_utils::file_utils::get_full_path;

#[derive(Debug)]
pub enum Compiler {
    Gcc,
    Gxx,
}

impl Compiler {
    pub fn cmd(&self) -> &'static str {
        match self {
            Compiler::Gcc => "gcc",
            Compiler::Gxx => "g++",
        }
    }

    pub fn args(&self, files: &Vec<std::path::PathBuf>) -> Vec<String> {
        match self {
            Compiler::Gcc => Compiler::collect_files_with_extension(files, &["c"]),
            Compiler::Gxx => Compiler::collect_files_with_extension(files, &["c", "cpp", "cc"]),
        }
    }

    fn collect_files_with_extension(
        files: &Vec<std::path::PathBuf>,
        allowed_extensions: &[&str],
    ) -> Vec<String> {
        files
            .iter()
            .filter_map(|file| {
                if Compiler::has_valid_extension(file, allowed_extensions) {
                    return get_full_path(file).ok();
                }
                None
            })
            .filter_map(|file| {
                if let Some(file_name) = file.to_str() {
                    return Some(String::from(file_name));
                }
                None
            })
            .collect()
    }
    fn has_valid_extension(file: &std::path::PathBuf, extensions: &[&str]) -> bool {
        if let Some(extension) = file.extension() {
            return extensions.contains(&extension.to_str().unwrap_or_default());
        }
        false
    }
}
