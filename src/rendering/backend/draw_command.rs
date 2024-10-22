use std::ops::Deref;

use crate::rendering::{
    shaders::{
        builder::ShaderBuilder,
        Bindings,
        ShaderInstance,
    },
    Color, ShaderConfig, Vertex, Vertex2D,
};

use super::{ DrawError, RenderPass, RenderPresentationSurface };

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
    ) -> Result<Self, DrawError> {
        device.push_error_scope(wgpu::ErrorFilter::Validation);
        let (surface_texture, surface_view)
            = presentation_surface
                .acquire_surface()
                .map_err(|e| DrawError::AcquirePresentationSurface(e))?;

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

    pub fn begin<'p, V, S, R>(
        &'p mut self,
        shader: &'p R,
        config: &ShaderConfig,
        label: wgpu::Label
    ) -> Result<RenderPass<'p, V>, DrawError> where
        V: Vertex,
        S: 'p + ShaderInstance + ?Sized,
        R: Deref<Target = S>,
    {
        let shader_context = self.shader_builder
            .get_mut_context(&shader.identifier())
            .ok_or_else(|| DrawError::ShaderNotFound { identifier: shader.identifier() })?;

        let bindings = shader.bindings(Bindings::new(
            &self.device,
            shader_context.bindings_descriptor().clone()
        )).map_err(|e| DrawError::BindingsFillFailed(e))?;

        Ok(RenderPass::new(
            self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label,
            }),
            &self.queue,
            &self.surface_view,
            &self.device,
            bindings,
            shader_context.pipeline(self.device, config)
        ))
    }

    pub fn present(self) {
        self.surface_texture.present();
    }

    pub fn clear<'p, C, S, R>(&'p mut self, color: C, shader: &'p R) -> Result<(), DrawError> where
        C: Into<Color<f32>>,
        S: ShaderInstance,
        R: Deref<Target = S>,
    {
        match self.begin::<Vertex2D, S, R>(shader, &ShaderConfig::default(), None) {
            Ok(pass) => pass.clear_color(color).submit().map_err(DrawError::from),
            Err(e) => Err(e),
        }
    }
}
