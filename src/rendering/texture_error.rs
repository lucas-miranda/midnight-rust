use std::{path::PathBuf, io};
use thiserror::Error;
use miette::Diagnostic;

#[derive(Error, Diagnostic, Debug)]
pub enum TextureError {
    #[error("failed to decode image at '{0}'")]
    UnsupportedFormat(PathBuf),

    #[error("failed to convert image to another representation")]
    RepresentationConversion,

    #[error("failed to open image: {0}")]
    Open(io::Error)
}
