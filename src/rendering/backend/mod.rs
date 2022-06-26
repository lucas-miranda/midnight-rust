mod render_backend_build_error;
pub use render_backend_build_error::RenderBackendBuildError;

mod render_backend;
pub use render_backend::{ExecutionContext, RenderBackend};

mod render_backend_builder;
pub use render_backend_builder::RenderBackendBuilder;

mod render_backend_operation_error;
pub use render_backend_operation_error::RenderBackendOperationError;

mod render_presentation_surface;
pub use render_presentation_surface::RenderPresentationSurface;

//#[cfg(feature = "no-backend")]
//use empty_backend as gfx_backend;

//#[cfg(feature = "vulkan-backend")]
//use vulkan_backend as gfx_backend;

#[repr(C)] //, align(256))]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
}
