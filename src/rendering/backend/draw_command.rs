use std::ops::Deref;

use wgpu::SurfaceError;

use crate::rendering::{
    shaders::{
        builder::ShaderBuilder,
        Bindings,
        ShaderInstance,
    },
    Color,
    ShaderConfig, Vertex,
};

use super::{
    RenderPass,
    RenderPresentationSurface,
};

pub struct DrawCommand<'a> {
    queue: &'a wgpu::Queue,
    surface_texture: wgpu::SurfaceTexture,
    surface_view: wgpu::TextureView,
    device: &'a wgpu::Device,
    shader_builder: &'a mut ShaderBuilder,
}

impl<'a> DrawCommand<'a> {
    pub(in crate::rendering) fn new(
        device: &'a wgpu::Device,
        queue: &'a wgpu::Queue,
        presentation_surface: &'a mut RenderPresentationSurface,
        shader_builder: &'a mut ShaderBuilder,
    ) -> Result<Self, SurfaceError> {
        device.push_error_scope(wgpu::ErrorFilter::Validation);
        let (surface_texture, surface_view) = presentation_surface.acquire_surface()?;

        Ok(Self {
            queue,
            surface_texture,
            surface_view,
            device,
            shader_builder,
        })
    }

    pub(in crate::rendering) fn device_queue(
        &self
    ) -> (&wgpu::Device, &wgpu::Queue) {
        (&self.device, &self.queue)
    }

    pub(in crate::rendering) fn shader_builder(&self) -> &ShaderBuilder {
        self.shader_builder
    }

    pub fn begin<'p, V, S>(
        &'p mut self,
        shader: &'p S,
        config: &ShaderConfig,
        label: wgpu::Label
    ) -> RenderPass<'p, V> where
        V: Vertex,
        S: Deref<Target = dyn ShaderInstance>,
    {
        let shader_context = self.shader_builder
            .get_mut_context(&shader.identifier())
            .unwrap();

        RenderPass::new(
            self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label,
            }),
            &self.queue,
            &self.surface_view,
            &self.device,
            shader.bindings(Bindings::new(
                &self.device,
                shader_context.bindings_descriptor()
            )),
            shader_context.pipeline(self.device, config)
        )
    }

    pub fn present(self) {
        self.surface_texture.present();
    }

    pub fn clear<'p, C, V, S>(&'p mut self, color: C, shader: &'p S) -> Result<(), super::RenderBackendOperationError> where
        C: Into<Color<f32>>,
        V: Vertex,
        S: Deref<Target = dyn ShaderInstance>,
    {
        self.begin::<V, _>(shader, &ShaderConfig::default(), None)
            .clear_color(color)
            .submit()
    }
}
