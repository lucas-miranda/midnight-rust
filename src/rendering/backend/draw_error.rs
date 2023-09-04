use thiserror::Error;
use miette::Diagnostic;

use crate::rendering::shaders::{Shader, BindingsError};

use super::{PassError, PresentationSurfaceError};

#[derive(Error, Diagnostic, Debug)]
pub enum DrawError {
    #[error("shader (0:?) not found at builder")]
    ShaderNotFound { identifier: Shader },

    #[error("pass failed: {0}")]
    Pass(#[from] PassError),

    #[error("acquiring presentation surface failed: {0}")]
    AcquirePresentationSurface(PresentationSurfaceError),

    #[error("shader failed to fill bindings: {0}")]
    BindingsFillFailed(BindingsError),
}
