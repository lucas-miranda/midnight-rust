use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};

#[derive(Debug)]
pub enum HALInitializeError {
    UnsupportedBackend(gfx_hal::UnsupportedBackend),
    SurfaceCreationFailed(gfx_hal::window::InitError),
    AdapterNotFound,
    QueueFamilyNotFound,
    LogicalDeviceCreationFailed(gfx_hal::device::CreationError),
    CommandQueueGroupNotFound,
    DeviceOutOfMemory(gfx_hal::device::OutOfMemory),
}

impl Error for HALInitializeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self {
            Self::UnsupportedBackend(base_err) => Some(base_err),
            Self::SurfaceCreationFailed(init_err) => Some(init_err),
            Self::LogicalDeviceCreationFailed(creation_err) => Some(creation_err),
            Self::DeviceOutOfMemory(device_err) => Some(device_err),
            _ => None,
        }
    }
}

impl Display for HALInitializeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            Self::UnsupportedBackend(base_err) => write!(f, "Failed to instantiate backend: {}", base_err),
            Self::SurfaceCreationFailed(init_err) => write!(f, "Failed to create surface for window: {}", init_err),
            Self::AdapterNotFound => write!(f, "Valid graphic adapter was not found."),
            Self::QueueFamilyNotFound => write!(f, "Valid queue family was not found."),
            Self::LogicalDeviceCreationFailed(creation_err) => write!(f, "Failed to create logical device: {}", creation_err),
            Self::CommandQueueGroupNotFound => write!(f, "Command queue group not found at gpu."),
            Self::DeviceOutOfMemory(device_err) => write!(f, "Operation failed by device being out of available memory: {}", device_err),
        }
    }
}
