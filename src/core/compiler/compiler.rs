use crate::core::file_utils::file_utils::get_full_path;

#[derive(Debug)]
pub enum Compiler {
    Gcc,
    Gxx,
}

impl Compiler {
    /// Returns the correct command based on the compiler
    pub fn cmd(&self) -> &'static str {
        match self {
            Compiler::Gcc => "gcc",
            Compiler::Gxx => "g++",
        }
    }

    /// Gets the correct arguments to launch the compiler
    pub fn args(
        &self,
        files: &Vec<std::path::PathBuf>,
        output_path: &std::path::PathBuf,
    ) -> Vec<String> {
        match self {
            Compiler::Gcc => Compiler::gxx_args(files, output_path, &["c"]),
            Compiler::Gxx => Compiler::gxx_args(files, output_path, &["c", "cpp", "cc"]),
        }
    }
    fn gxx_args(
        files: &Vec<std::path::PathBuf>,
        output_path: &std::path::PathBuf,
        extensions: &[&str],
    ) -> Vec<String> {
        let path = output_path.to_str().unwrap_or("");
        let mut args = Compiler::collect_files_with_extension(files, extensions);
        args.extend([
            String::from("-fdiagnostics-color=always"),
            String::from("-o"),
            String::from(path),
        ]);
        return args;
    }

    /// Collects the files in `files` that have an extension found in `allowed_extensions`
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
    /// Checks if the `file`'s extension is contained in `extensions`
    fn has_valid_extension(file: &std::path::PathBuf, extensions: &[&str]) -> bool {
        if let Some(extension) = file.extension() {
            return extensions.contains(&extension.to_str().unwrap_or_default());
        }
        false
    }
}
