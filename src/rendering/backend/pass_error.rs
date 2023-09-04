use thiserror::Error;
use miette::Diagnostic;

use crate::rendering::shaders::BindingsError;

#[derive(Error, Diagnostic, Debug)]
pub enum PassError {
    #[error("bindings failed: {0}")]
    Bindings(#[from] BindingsError),

    #[error("failed at submission: {0}")]
    Submit(#[from] wgpu::Error),
}
