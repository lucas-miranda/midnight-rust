use thiserror::Error;
use miette::Diagnostic;
use winit::error::EventLoopError;

use crate::window::WindowError;

#[non_exhaustive]
#[derive(Error, Diagnostic, Debug)]
pub enum ApplicationError {
    #[error("window creation has failed")]
    WindowCreationFailed(EventLoopError),

    #[error("window failed")]
    WindowError(#[from] WindowError)
}
