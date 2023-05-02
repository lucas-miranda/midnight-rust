use std::slice::Iter;
use wgpu::util::DeviceExt;

use crate::{
    rendering::{
        shaders::builder::ShaderContext,
        Color,
        DrawConfig,
        RenderState,
    },
    math::Vector2,
};

pub struct RenderPass<'a> {
    encoder: wgpu::CommandEncoder,
    queue: &'a wgpu::Queue,
    device: &'a wgpu::Device,
    surface_view: &'a wgpu::TextureView,
    vertex_data: Vec<Vector2<f32>>,
    bind_group: Option<wgpu::BindGroup>,
    shader_context: &'a ShaderContext,
    clear_color: Option<Color<f32>>,
}

impl<'a> RenderPass<'a> {
    pub(super) fn new(
        encoder: wgpu::CommandEncoder,
        queue: &'a wgpu::Queue,
        surface_view: &'a wgpu::TextureView,
        device: &'a wgpu::Device,
        bind_group: Option<wgpu::BindGroup>,
        shader_context: &'a ShaderContext,
    ) -> Self {
        Self {
            encoder,
            queue,
            device,
            surface_view,
            vertex_data: Vec::new(),
            bind_group,
            shader_context,
            clear_color: None,
        }
    }

    pub fn clear_color<C: Into<Color<f32>>>(mut self, color: C) -> Self {
        self.clear_color = Some(color.into());

        self
    }

    /*
    pub fn extend_vertices<T: IntoIterator<Item = Vector2<f32>>>(mut self, iter: T) -> Self {
        self.vertex_data.extend(iter);

        self
    }
    */

    pub fn submit(mut self) -> Result<(), super::RenderBackendOperationError> {
        /*
        if !self.vertex_data.is_empty() {
            self.vertex_data
                .iter_mut()
                .for_each(|v| *v += self.config.position);
        }
        */

        let vertex_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("vertex buffer"),
            contents: bytemuck::cast_slice(self.vertex_data.as_slice()),
            usage: wgpu::BufferUsages::VERTEX,
        });

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

            pass.set_pipeline(&self.shader_context.pipeline);

            if let Some(ref bindings) = self.bind_group {
                pass.set_bind_group(0, bindings, &[]);
            }

            pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            pass.draw(0..(self.vertex_data.len() as u32), 0..1);
        }

        // TODO  try to submit multiple command buffers at once?
        self.queue.submit(Some(self.encoder.finish()));

        Ok(())
    }
}

impl<'a> RenderState for RenderPass<'a> {
    fn extend(&mut self, vertices: Iter<Vector2<f32>>, draw_config: DrawConfig) {
        self.vertex_data.extend(
            vertices.into_iter()
                    .map(|v| *v + draw_config.position)
        );
    }
}
