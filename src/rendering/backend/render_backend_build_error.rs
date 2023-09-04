use thiserror::Error;
use miette::Diagnostic;

#[derive(Error, Diagnostic, Debug)]
pub enum RenderBackendBuildError {
    /*
    #[error("failed to instantiate backend: {0}")]
    InstanceFailed(wgpu::InstanceError),
    */

    #[error("failed to create surface: {0}")]
    SurfaceFailed(#[from] wgpu::CreateSurfaceError),

    #[error("valid graphic adapter was not found")]
    AdapterNotFound,

    #[error("presentation is not supported")]
    PresentationNotSupported,

    #[error("failed to open logical device: {0}")]
    LogicalDeviceOpenFailed(#[from] wgpu::RequestDeviceError),

    /*
    #[error("failed to configure surface: {0}")]
    SurfaceConfigureFailed(wgpu::SurfaceError),

    #[error("failed to create pipeline layout: {0}")]
    PipelineLayoutFailed(wgpu::DeviceError),

    #[error("failed to create pipeline: {0}")]
    PipelineFailed(wgpu::PipelineError),

    #[error("failed to create command encoder: {0}")]
    CommandEncoderFailed(wgpu::DeviceError),
    */
}
