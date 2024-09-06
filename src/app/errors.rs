use core::fmt;

/// Represents a compilation start error
pub(super) enum CompilationStartError {
    CompilerNotSupported,
    ErrorStartingCompileProcess,
    BuildFolderGenerationFailed,
}
impl fmt::Display for CompilationStartError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompilationStartError::CompilerNotSupported => write!(f, "Compiler not supported"),
            CompilationStartError::ErrorStartingCompileProcess => {
                write!(f, "Compiler could not be stated")
            }
            CompilationStartError::BuildFolderGenerationFailed => {
                write!(f, "Build folder generation fail")
            }
        }
    }
}

/// Represents a start exo error
/// See `App::start_exo`
pub(super) enum StartExoFail {
    CouldNotStartCompilation(CompilationStartError),
}
impl fmt::Display for StartExoFail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StartExoFail::CouldNotStartCompilation(err) => write!(f, "{}", err),
        }
    }
}
