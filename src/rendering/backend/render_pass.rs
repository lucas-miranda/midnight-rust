use wgpu::util::DeviceExt;

use crate::{
    rendering::{
        shaders::{
            Shader,
            builder::ShaderBuilder
        },
        Color,
        DrawConfig,
    },
    math::Vector2,
};

pub struct RenderPass<'a> {
    encoder: &'a mut wgpu::CommandEncoder,
    device: &'a wgpu::Device,
    surface_view: &'a wgpu::TextureView,
    vertex_data: Vec<Vector2<f32>>,
    bind_group: Option<wgpu::BindGroup>,
    shader_builder: &'a ShaderBuilder,
    shader: Option<&'a Shader>,
    config: &'a DrawConfig,
    clear_color: Option<Color<f32>>,
}

impl<'a> RenderPass<'a> {
    pub fn new(
        encoder: &'a mut wgpu::CommandEncoder,
        surface_view: &'a wgpu::TextureView,
        device: &'a wgpu::Device,
        shader_builder: &'a ShaderBuilder,
        vertex_data: Vec<Vector2<f32>>,
        config: &'a DrawConfig,
    ) -> Self {
        Self {
            encoder,
            device,
            surface_view,
            vertex_data,
            bind_group: None,
            shader_builder,
            shader: None,
            config,
            clear_color: None,
        }
    }

    pub fn clear_color<C: Into<Color<f32>>>(mut self, color: C) -> Self {
        self.clear_color = Some(color.into());

        self
    }

    pub fn using_shader<U: bytemuck::Zeroable + bytemuck::Pod + bytemuck::NoUninit>(mut self, shader: &'a Shader, uniforms: Option<&U>) -> Self {
        self.shader = Some(shader);

        self.bind_group = if let Some(uni) = uniforms {
            let uniform_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("uniforms buffer"),
                contents: bytemuck::cast_slice(&[*uni]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

            let shader_context = self.shader_builder
                .get_context(&shader.id())
                .unwrap();

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
        } else {
            let shader_context = self.shader_builder
                .get_context(&shader.id())
                .unwrap();

            let bind_group = Some(
                self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("Uniform Bind Group"),
                    layout: &shader_context.bind_group_layout,
                    entries: &[
                    ],
                })
            );

            bind_group
        };

        self
    }

    pub fn submit(mut self) -> Result<(), super::RenderBackendOperationError> {
        let vertex_buffer = if self.vertex_data.is_empty() {
            None
        } else {
            self.vertex_data
                .iter_mut()
                .for_each(|v| *v += self.config.position);

            Some(self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("vertex buffer"),
                contents: bytemuck::cast_slice(self.vertex_data.as_slice()),
                usage: wgpu::BufferUsages::VERTEX,
            }))
        };

        // create wgpu render pass and submit

        {
            let load = match self.clear_color {
                Some(clear_color) => wgpu::LoadOp::Clear(clear_color.into()),
                None => wgpu::LoadOp::Load,
            };

            let mut pass = self.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: self.surface_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load,
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            if let Some(shader) = self.shader {
                let shader_context = self.shader_builder
                    .get_context(&shader.id())
                    .unwrap();

                pass.set_pipeline(&shader_context.pipeline);
            }

            if let Some(ref bindings) = self.bind_group {
                pass.set_bind_group(0, bindings, &[]);
            }

            if let Some(ref v_buffer) = vertex_buffer {
                pass.set_vertex_buffer(0, v_buffer.slice(..));
            }

            pass.draw(0..(self.vertex_data.len() as u32), 0..1);
        }

        Ok(())
    }
}
