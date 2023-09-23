use thiserror::Error;
use miette::Diagnostic;

use super::{
    backend::{DrawError, PassError},
    shaders::{BindingsError, Shader}, RenderStateError,
};

#[derive(Error, Diagnostic, Debug)]
pub enum DrawBatcherError {
    #[error("draw command failed")]
    DrawCommand(#[from] DrawError),

    #[error("shader {1:?} -> bindings failed: {0}")]
    Bindings(BindingsError, Shader),

    #[error("pass submission failed: {0}")]
    PassSubmit(#[from] PassError),

    #[error("render state failed: {0}")]
    RenderState(#[from] RenderStateError)
}
