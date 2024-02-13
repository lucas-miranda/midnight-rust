use thiserror::Error;
use miette::Diagnostic;
use winit::error::EventLoopError;

#[non_exhaustive]
#[derive(Error, Diagnostic, Debug)]
pub enum WindowError {
    #[error("window event loop raised an error")]
    EventLoopError(#[from] EventLoopError),

}
