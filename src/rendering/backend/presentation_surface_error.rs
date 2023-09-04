use thiserror::Error;
use miette::Diagnostic;
use wgpu::SurfaceError;

#[derive(Error, Diagnostic, Debug)]
pub enum PresentationSurfaceError {
    #[error("graphics device reference was lost")]
    DeviceLost,

    #[error("acquire surface texture failed: {0}")]
    AcquireSurfaceTexture(SurfaceError),
}
