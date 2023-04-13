use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};

#[derive(Debug)]
pub enum RenderBackendBuildError {
    /*
    InstanceFailed(wgpu::InstanceError),
    */
    SurfaceFailed(wgpu::CreateSurfaceError),
    AdapterNotFound,
    PresentationNotSupported,
    LogicalDeviceOpenFailed(wgpu::RequestDeviceError),
    /*
    SurfaceConfigureFailed(wgpu::SurfaceError),
    PipelineLayoutFailed(wgpu::DeviceError),
    PipelineFailed(wgpu::PipelineError),
    CommandEncoderFailed(wgpu::DeviceError),
    */
}

impl Error for RenderBackendBuildError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self {
            //Self::InstanceFailed(base_err) => Some(base_err),
            Self::SurfaceFailed(base_err) => Some(base_err),
            Self::LogicalDeviceOpenFailed(device_err) => Some(device_err),
            /*
            Self::SurfaceConfigureFailed(surface_err) => Some(surface_err),
            Self::PipelineLayoutFailed(device_err) => Some(device_err),
            Self::PipelineFailed(pipeline_err) => Some(pipeline_err),
            Self::CommandEncoderFailed(device_err) => Some(device_err),
            */
            _ => None,
        }
    }
}

impl Display for RenderBackendBuildError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            /*
            Self::InstanceFailed(base_err)
                => write!(f, "Failed to instantiate backend: {}", base_err),
            */
            Self::SurfaceFailed(base_err)
                => write!(f, "Failed to create surface: {}", base_err),
            Self::AdapterNotFound => write!(f, "Valid graphic adapter was not found."),
            Self::PresentationNotSupported => write!(f, "Presentation is not supported."),
            Self::LogicalDeviceOpenFailed(device_err)
                => write!(f, "Failed to open logical device: {}", device_err),
            /*
            Self::SurfaceConfigureFailed(surface_err)
                => write!(f, "Failed to configure surface: {}", surface_err),
            Self::PipelineLayoutFailed(device_err)
                => write!(f, "Failed to create pipeline layout: {}", device_err),
            Self::PipelineFailed(pipeline_err)
                => write!(f, "Failed to create pipeline: {}", pipeline_err),
            Self::CommandEncoderFailed(device_err)
                => write!(f, "Failed to create command encoder: {}", device_err),
            */
        }
    }
}
