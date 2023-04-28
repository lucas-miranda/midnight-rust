use std::rc::Rc;

use crate::{
    math::Vector2,
    rendering::{
        shaders::builder::ShaderBuilder,
        DrawConfig,
    },
};

use super::{
    DrawCommand,
    RenderPass,
    RenderPresentationSurface,
};

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

    /*
    pub fn device(&self) -> &Rc<wgpu::Device> {
        &self.device
    }

    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    pub fn presentation_surface(&self) -> &RenderPresentationSurface {
        &self.presentation_surface
    }

    pub fn mut_presentation_surface(&mut self) -> &mut RenderPresentationSurface {
        &mut self.presentation_surface
    }

    pub fn shader_builder(&mut self) -> &mut ShaderBuilder {
        &mut self.shader_builder
    }
    */

    /*
    pub fn draw_vertices<'d>(
        &'d mut self,
        vertices: Vec<Vector2<f32>>,
        config: &'d DrawConfig,
    ) -> RenderPass<'d> {
        RenderPass::new(
            &self.device,
            &self.queue,
            &mut self.presentation_surface,
            &self.shader_builder,
            vertices,
            config,
        )
    }
    */
}
