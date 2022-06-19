use crate::window::Window;
use super::{
    RenderBackend,
    RenderBackendBuilder,
    GraphicBackendInitError,
};

type ActiveApi = wgpu_hal::api::Vulkan;

pub type Result<T> = std::result::Result<T, GraphicBackendInitError>;

/// User interface to interact with graphics backend.
pub struct GraphicBackend {
    render_backend: RenderBackend<ActiveApi>,
}

impl GraphicBackend {
    pub fn new(window: &Window, surface_size: (u32, u32)) -> Result<Self> {
        let render_backend = RenderBackendBuilder::new(window, surface_size)
            .build()
            .unwrap();
            //.map_err(GraphicBackendInitError::HALInitFailed)?;

        Ok(Self {
            render_backend,
        })
    }

    pub fn render_backend(&self) -> &RenderBackend<ActiveApi> {
        &self.render_backend
    }

    pub fn mut_render_backend(&mut self) -> &mut RenderBackend<ActiveApi> {
        &mut self.render_backend
    }
}
