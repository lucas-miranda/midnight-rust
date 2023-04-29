use std::rc::Rc;

use crate::rendering::shaders::builder::ShaderBuilder;
use super::RenderPresentationSurface;

pub struct RenderBackend {
    pub device: Rc<wgpu::Device>,
    pub queue: wgpu::Queue,
    pub presentation_surface: RenderPresentationSurface,
    pub shader_builder: ShaderBuilder,
}

impl RenderBackend {
    pub(super) fn new(
        device: Rc<wgpu::Device>,
        queue: wgpu::Queue,
        presentation_surface: RenderPresentationSurface,
    ) -> Self {
        let shader_builder = ShaderBuilder::new(
            Rc::downgrade(&device),
            *presentation_surface.surface_format(),
        );

        Self {
            device,
            queue,
            presentation_surface,
            shader_builder,
        }
    }
}
