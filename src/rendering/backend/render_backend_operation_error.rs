use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};

#[derive(Debug)]
pub enum RenderBackendOperationError {
    /*
    WaitFailed(gfx_hal::device::WaitError),
    DeviceOutOfMemory(gfx_hal::device::OutOfMemory),
    AcquireImageFailed(gfx_hal::window::AcquireError),
    FramePresentFailed(gfx_hal::window::PresentError),
    */
}

/*
impl Error for RenderPipelineOperationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self {
            Self::WaitFailed(wait_err) => Some(wait_err),
            Self::DeviceOutOfMemory(device_err) => Some(device_err),
            Self::AcquireImageFailed(acquire_err) => Some(acquire_err),
            Self::FramePresentFailed(present_err) => Some(present_err),
        }
    }
}

impl Display for RenderPipelineOperationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            Self::WaitFailed(wait_err) => write!(f, "Fail occurred when waiting: {}", wait_err),
            Self::DeviceOutOfMemory(device_err) => write!(f, "Operation failed by device being out of available memory: {}", device_err),
            Self::AcquireImageFailed(acquire_err) => write!(f, "Failed to acquire image from surface: {}", acquire_err),
            Self::FramePresentFailed(present_err) => write!(f, "Failed to present surface image: {}", present_err),
        }
    }
}
*/
