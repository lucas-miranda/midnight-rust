use std::rc::Rc;

use crate::{
    rendering::shaders::builder::ShaderBuilder,
    math::Vec2,
};

use super::{
    DrawCommand,
    RenderBackendOperationError,
    RenderPresentationSurface,
};

pub type Result<T> = std::result::Result<T, RenderBackendOperationError>;

pub struct RenderContext {
    pub surface_texture: wgpu::SurfaceTexture,
    pub surface_view: wgpu::TextureView,
}

pub struct RenderBackend {
    instance: wgpu::Instance,
    device: Rc<wgpu::Device>,
    capabilities: wgpu::SurfaceCapabilities,
    queue: wgpu::Queue,
    presentation_surface: RenderPresentationSurface,
    shader_builder: ShaderBuilder,
}

impl RenderBackend {
    pub(super) fn new(
        instance: wgpu::Instance,
        device: Rc<wgpu::Device>,
        capabilities: wgpu::SurfaceCapabilities,
        queue: wgpu::Queue,
        presentation_surface: RenderPresentationSurface,
    ) -> Self {
        let shader_builder = ShaderBuilder::new(
            Rc::downgrade(&device),
            *presentation_surface.surface_format(),
        );

        Self {
            instance,
            device,
            capabilities,
            queue,
            presentation_surface,
            shader_builder,
        }
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

    pub fn draw_vertices<'d>(
        &'d mut self,
        vertices: &[Vec2<f32>],
    ) -> DrawCommand<'d> {
        DrawCommand::new(
            &self.device,
            &self.capabilities,
            &self.queue,
            &mut self.presentation_surface,
            &self.shader_builder,
            vertices,
        )
    }

    pub(super) fn prepare_render(&mut self) -> Result<RenderContext> {
        // reconfigure if needed
        self.presentation_surface.reconfigure_swapchain(false);

        let surface_texture = self.presentation_surface
            .surface()
            .get_current_texture()
            .unwrap();

        let surface_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        Ok(RenderContext {
            surface_texture,
            surface_view,
        })
    }

    pub(super) fn submit(
        &mut self,
        render_context: RenderContext,
        encoder: wgpu::CommandEncoder
    ) -> Result<()> {
        let RenderContext {
            surface_texture,
            surface_view,
        } = render_context;

        //let context = &mut self.contexts[execution_context_index];
        self.queue.submit(Some(encoder.finish()));
        surface_texture.present();

        Ok(())
    }
}
