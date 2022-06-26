use num_traits::Num;
use crate::{
    math::Vec2,
    rendering::shaders::Shader,
    window::Window,
};

use super::{
    ActiveApi,
    GraphicAdapterInitError,
    backend::{
        RenderBackend,
        RenderBackendBuilder,
        Vertex,
    },
    shaders::builder::ShaderBuilder,
};

pub type Result<T> = std::result::Result<T, super::GraphicAdapterInitError>;

pub struct GraphicAdapter {
    backend: RenderBackend<ActiveApi>,
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

    pub fn shader_builder(&mut self) -> &mut ShaderBuilder<ActiveApi> {
        self.backend.shader_builder()
    }

    pub fn draw_vertices<T: Num>(&mut self, vertices: &[Vec2<T>], shader: &Shader) {
        self.backend.draw_vertices(vertices, shader).unwrap();
    }

    /*
    pub fn render(&mut self) {
        self.backend
            .render()
            .unwrap()
    }
    */
}
