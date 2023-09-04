use thiserror::Error;
use miette::Diagnostic;

use super::shaders::Shader;

#[derive(Error, Diagnostic, Debug)]
pub enum RenderStateError {
    #[error("expected shader config to be defined")]
    MissingShaderConfig,

    #[error("Shader ({0:?}) isn't registered.")]
    ShaderNotFound(Shader),
}
