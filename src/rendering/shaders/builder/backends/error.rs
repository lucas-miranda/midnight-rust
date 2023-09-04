use thiserror::Error;
use miette::Diagnostic;

#[derive(Error, Diagnostic, Debug)]
pub enum ShaderBackendError {
    #[cfg(feature = "shader-shaderc")]
    #[error("shader compilation failed with {err_count} errors: {}", detailed)]
    CompilationFailed { err_count: u32, detailed: String },

    #[cfg(feature = "shader-shaderc")]
    #[error("internal error: {0}")]
    InternalError(String),

    #[cfg(feature = "shader-shaderc")]
    #[error("invalid stage: {0}")]
    InvalidStage(String),

    #[cfg(feature = "shader-shaderc")]
    #[error("invalid assembly: {0}")]
    InvalidAssembly(String),

    #[error("null result object: {0}")]
    NullResultObject(String),
}
