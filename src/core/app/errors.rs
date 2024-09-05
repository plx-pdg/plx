use core::fmt;

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

pub(super) enum StartExoFail {
    CouldNotLaunchEditor,
    CouldNotStartCompilation(CompilationStartError),
}
impl fmt::Display for StartExoFail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StartExoFail::CouldNotLaunchEditor => write!(f, "Error launching editor"),
            StartExoFail::CouldNotStartCompilation(err) => write!(f, "{}", err),
        }
    }
}
