use std::slice::Iter;
use wgpu::util::DeviceExt;

use crate::rendering::{
    shaders::{ builder::ShaderPipeline, Bindings },
    Color, DrawConfig, RenderState, RenderStateError,
    Texture, Vertex,
};

use super::PassError;

pub struct RenderPass<'a, V> where
    V: Vertex
{
    encoder: wgpu::CommandEncoder,
    queue: &'a wgpu::Queue,
    device: &'a wgpu::Device,
    surface_view: &'a wgpu::TextureView,
    vertex_data: Vec<V>,
    bindings: Bindings<'a>,
    shader_pipeline: &'a ShaderPipeline,
    clear_color: Option<Color<f32>>,
}

impl<'a, V: Vertex> RenderPass<'a, V> {
    pub(super) fn new(
        encoder: wgpu::CommandEncoder,
        queue: &'a wgpu::Queue,
        surface_view: &'a wgpu::TextureView,
        device: &'a wgpu::Device,
        bindings: Bindings<'a>,
        shader_pipeline: &'a ShaderPipeline,
    ) -> Self {
        Self {
            encoder,
            queue,
            device,
            surface_view,
            vertex_data: Vec::new(),
            bindings,
            shader_pipeline,
            clear_color: None,
        }
    }

    pub(in crate::rendering) fn bindings(&mut self) -> &mut Bindings<'a> {
        &mut self.bindings
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

    pub fn submit(mut self) -> Result<(), PassError> {
        self.device.push_error_scope(wgpu::ErrorFilter::Validation);

        let bind_group = {
            let bindings = self.bindings.collect().map_err(PassError::from)?;

            let bind_group = Some(
                self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("Uniform Bind Group"),
                    layout: &self.shader_pipeline.get_bind_group_layout(0),
                    entries: &bindings,
                })
            );

            bind_group
        };

        let vertex_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("vertex buffer"),
            contents: bytemuck::cast_slice(self.vertex_data.as_slice()),
            usage: wgpu::BufferUsages::VERTEX,
        });

        // create wgpu render pass and submit

        {
            /*
            let load = match self.clear_color {
                Some(clear_color) => wgpu::LoadOp::Clear(clear_color.into()),
                None => wgpu::LoadOp::Load,
            };
            */

            let mut pass = self.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: self.surface_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(Color::<f32>::WHITE.into()),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            pass.push_debug_group("Prepare data for draw.");

            if let Some(ref bindings) = bind_group {
                pass.set_bind_group(0, bindings, &[]);
            }

            pass.set_pipeline(self.shader_pipeline);

            pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            pass.pop_debug_group();
            pass.insert_debug_marker("Draw!");

            pass.draw(0..(self.vertex_data.len() as u32), 0..1);
        }

        // TODO  try to submit multiple command buffers at once?
        self.queue.submit(Some(self.encoder.finish()));

        //let ex = async_executor::Executor::new();
        //let task = ex.spawn(self.device.pop_error_scope());
        //futures_lite::future::block_on(ex.run());

        match futures_lite::future::block_on(self.device.pop_error_scope()) {
            Some(err) => Err(PassError::from(err)),
            None => Ok(()),
        }?;

        Ok(())
    }
}

impl<'a, V: Vertex> RenderState<V> for RenderPass<'a, V> {
    fn extend<'t>(
        &mut self,
        vertices: Iter<V>,
        _texture: Option<&'t Texture>,
        draw_config: DrawConfig<V>,
    ) -> Result<(), RenderStateError> {
        self.vertex_data.extend(
            vertices.into_iter()
                    .map(|v| *v + draw_config.vertex)
        );

        Ok(())
    }
}
