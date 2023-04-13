mod render_backend_build_error;
pub use render_backend_build_error::RenderBackendBuildError;

mod render_backend;
pub use render_backend::RenderBackend;

mod render_backend_builder;
pub use render_backend_builder::RenderBackendBuilder;

mod render_backend_operation_error;
pub use render_backend_operation_error::RenderBackendOperationError;

mod render_presentation_surface;
pub use render_presentation_surface::RenderPresentationSurface;

mod draw_command;
pub use draw_command::*;

#[repr(C)] //, align(256))]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
}
