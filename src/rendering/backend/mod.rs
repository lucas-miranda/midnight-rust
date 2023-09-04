mod render_backend_build_error;
pub use render_backend_build_error::RenderBackendBuildError;

mod render_backend;
pub use render_backend::RenderBackend;

mod render_backend_builder;
pub use render_backend_builder::RenderBackendBuilder;

mod render_presentation_surface;
pub use render_presentation_surface::RenderPresentationSurface;

mod presentation_surface_error;
pub use presentation_surface_error::PresentationSurfaceError;

mod draw_command;
pub use draw_command::*;

mod draw_error;
pub use draw_error::DrawError;

mod render_pass;
pub use render_pass::RenderPass;

mod pass_error;
pub use pass_error::PassError;

#[repr(C)] //, align(256))]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
}
