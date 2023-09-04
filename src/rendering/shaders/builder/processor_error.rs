use thiserror::Error;
use miette::Diagnostic;

use super::ShaderFormat;

#[derive(Error, Diagnostic, Debug)]
pub enum ShaderProcessorError {
    #[error("support for shader format {0:?} isn't enabled.")]
    FormatNotSupported(ShaderFormat),
}
