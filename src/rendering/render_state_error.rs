use thiserror::Error;
use miette::Diagnostic;

use super::shaders::{Shader, BindingsError};

#[derive(Error, Diagnostic, Debug)]
pub enum RenderStateError {
    #[error("expected shader config to be defined")]
    MissingShaderConfig,

    #[error("Shader ({0:?}) isn't registered.")]
    ShaderNotFound(Shader),

    #[error("bindings failed: {0}")]
    Bindings(#[from] BindingsError),

    #[error("Shader ({0:?}) instance not found.")]
    ShaderInstanceNotFound(Shader),
}
