use std::{
    mem,
    rc::Rc,
};

use wgpu::util::DeviceExt;

use crate::{
    rendering::{
        backend::RenderPresentationSurface,
        shaders::{
            Shader,
            builder::ShaderBuilder
        },
    },
    math::Vec2,
};

pub struct DrawCommand<'a> {
    device: &'a Rc<wgpu::Device>,
    capabilities: &'a wgpu::SurfaceCapabilities,
    queue: &'a wgpu::Queue,
    presentation_surface: &'a mut RenderPresentationSurface,
    vertices_count: u32,
    vertex_buffer: wgpu::Buffer,
    vertex_buffer_size: Option<wgpu::BufferSize>,
    shader_builder: &'a ShaderBuilder,
    shader: Option<&'a Shader>,
    bind_group: Option<wgpu::BindGroup>,
}

impl<'a> DrawCommand<'a> {
    pub fn new(
        device: &'a Rc<wgpu::Device>,
        capabilities: &'a wgpu::SurfaceCapabilities,
        queue: &'a wgpu::Queue,
        presentation_surface: &'a mut RenderPresentationSurface,
        shader_builder: &'a ShaderBuilder,
        vertices: &[Vec2<f32>],
    ) -> Self {
            let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("vertex buffer"),
                contents: bytemuck::cast_slice(vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });

        Self {
            device,
            capabilities,
            queue,
            presentation_surface,
            vertices_count: vertices.len() as u32,
            vertex_buffer,
            vertex_buffer_size: wgpu::BufferSize::new(mem::size_of::<Vec2<f32>>() as _),
            shader_builder,
            shader: None,
            bind_group: None,
        }
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
            None
        };

        self
    }

    pub fn submit(mut self) -> Result<(), super::RenderBackendOperationError> {
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("main command encoder"),
        });

        self.device.push_error_scope(wgpu::ErrorFilter::Validation);
        let (surface_texture, surface_view) = self.prepare_render();

        if let Some(bindings) = self.bind_group {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &surface_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
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

            //if let Some(bindings) = self.bind_group {
                pass.set_bind_group(0, &bindings, &[]);
            //}

            pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            pass.draw(0..self.vertices_count, 0..1);
        }

        //self.render_backend.submit(render_context, encoder)?;
        self.queue.submit(Some(encoder.finish()));
        surface_texture.present();

        Ok(())
    }

    fn prepare_render(&mut self) -> (wgpu::SurfaceTexture, wgpu::TextureView) {
        // reconfigure if needed
        self.presentation_surface.reconfigure_swapchain(false);

        let surface_texture = self.presentation_surface
            .surface()
            .get_current_texture()
            .unwrap();

        let surface_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        (surface_texture, surface_view)
    }
}
