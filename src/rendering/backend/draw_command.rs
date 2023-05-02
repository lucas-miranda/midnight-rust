use wgpu::SurfaceError;
use wgpu::util::DeviceExt;

use crate::rendering::{
    shaders::{
        builder::ShaderBuilder,
        ShaderInstance,
    },
    Color,
    DrawConfig,
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
    shader_builder: &'a ShaderBuilder,
}

impl<'a> DrawCommand<'a> {
    pub(in crate::rendering) fn new(
        device: &'a wgpu::Device,
        queue: &'a wgpu::Queue,
        presentation_surface: &'a mut RenderPresentationSurface,
        shader_builder: &'a ShaderBuilder,
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

    pub fn begin<'p>(&'p mut self, shader: &'p dyn ShaderInstance, label: wgpu::Label) -> RenderPass<'p> {
        //let shader = shader_ref.as_ref();

        let shader_context = self.shader_builder
            .get_context(&shader.id())
            .unwrap();

        let bind_group = {
            let uniform_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("uniforms buffer"),
                contents: shader.uniforms_as_slice(),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

            let bind_group = Some(
                self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("Uniform Bind Group"),
                    layout: &shader_context.bind_group_layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: uniform_buffer.as_entire_binding(),
                        }
                    ],
                })
            );

            bind_group
        };

        RenderPass::new(
            self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label,
            }),
            &self.queue,
            &self.surface_view,
            &self.device,
            bind_group,
            shader_context,
        )
    }

    pub fn present(self) {
        self.surface_texture.present();
    }

    pub fn clear<'p, C: Into<Color<f32>>>(&'p mut self, color: C, shader: &'p dyn ShaderInstance) -> Result<(), super::RenderBackendOperationError> {
        self.begin(shader, None)
            .clear_color(color)
            .submit()
    }
}
