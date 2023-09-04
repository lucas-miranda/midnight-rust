use thiserror::Error;
use miette::Diagnostic;

use crate::rendering::RenderStateError;

#[derive(Error, Diagnostic, Debug)]
pub enum GraphicDrawError {
    #[error("render state failed: {0}")]
    RenderState(#[from] RenderStateError),
}
