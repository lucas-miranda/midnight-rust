use wgpu::SurfaceError;

use crate::window::Window;

use super::{
    backend::{
        DrawCommand,
        RenderBackend,
        RenderBackendBuilder,
    },
    shaders::builder::ShaderBuilder,
    GraphicAdapterInitError,
};

pub type Result<T> = std::result::Result<T, super::GraphicAdapterInitError>;

pub struct GraphicAdapter {
    backend: RenderBackend,
}

impl GraphicAdapter {
    pub fn new(window: &Window) -> Result<Self> {
        Self::with_surface_size(window, (320, 180))
    }

    pub fn with_surface_size(window: &Window, size: (u32, u32)) -> Result<Self> {
        let backend = RenderBackendBuilder::new(window, size)
            .build()
            .map_err(GraphicAdapterInitError::BackendFailed)?;

        Ok(Self {
            backend,
        })
    }

    pub fn request_resize_surface(&mut self, width: u32, height: u32) {
        self.backend
            .presentation_surface
            .request_reconfigure_swapchain_with(width, height);
    }

    pub fn shader_builder(&mut self) -> &mut ShaderBuilder {
        &mut self.backend.shader_builder
    }

    pub fn prepare_draw(&mut self) -> std::result::Result<DrawCommand, SurfaceError> {
        DrawCommand::new(
            &self.backend.device,
            &self.backend.queue,
            &mut self.backend.presentation_surface,
            &self.backend.shader_builder,
        )
    }
}
