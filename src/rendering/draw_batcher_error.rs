use thiserror::Error;
use miette::Diagnostic;

use super::{
    backend::{DrawError, PassError},
    shaders::BindingsError, RenderStateError,
};

#[derive(Error, Diagnostic, Debug)]
pub enum DrawBatcherError {
    #[error("draw command failed")]
    DrawCommand(#[from] DrawError),

    #[error("bindings failed: {0}")]
    Bindings(#[from] BindingsError),

    #[error("pass submission failed: {0}")]
    PassSubmit(#[from] PassError),

    #[error("render state failed: {0}")]
    RenderState(#[from] RenderStateError)
}
