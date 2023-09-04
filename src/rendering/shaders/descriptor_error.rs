use thiserror::Error;
use miette::Diagnostic;

use super::{ShaderStageKind, builder::ShaderProcessorError};

#[derive(Error, Diagnostic, Debug)]
pub enum ShaderDescriptorError {
    #[error("there is no stage {0} registered")]
    StageNotFound(ShaderStageKind),

    #[error("shader processor failed")]
    ProcessorFailed(#[from] ShaderProcessorError),
}
