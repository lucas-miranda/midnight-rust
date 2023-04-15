use crate::{
    math::Vector2,
    window::Window,
};

use super::{
    backend::{
        RenderBackend,
        RenderBackendBuilder, DrawCommand,
    },
    shaders::builder::ShaderBuilder,
    GraphicAdapterInitError,
    DrawConfig,
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
            .mut_presentation_surface()
            .request_reconfigure_swapchain_with(width, height);
    }

    pub fn shader_builder(&mut self) -> &mut ShaderBuilder {
        self.backend.shader_builder()
    }

    pub fn draw_vertices<'d>(
        &'d mut self,
        vertices: Vec<Vector2<f32>>,
        config: &'d DrawConfig,
    ) -> DrawCommand<'d> {
        self.backend.draw_vertices(vertices, config)
    }

    /*
    pub fn render(&mut self) {
        self.backend
            .render()
            .unwrap()
    }
    */
}
