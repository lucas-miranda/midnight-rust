use wgpu::SurfaceError;

use crate::{
    math::{Vector2, Tri},
    rendering::{
        shaders::builder::ShaderBuilder,
        Color,
        DrawConfig,
        graphics::Graphic,
    },
};

use super::{
    RenderPass,
    RenderPresentationSurface,
};

pub struct DrawCommand<'a> {
    encoder: Option<wgpu::CommandEncoder>,
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
            encoder: None,
            queue,
            surface_texture,
            surface_view,
            device,
            shader_builder,
        })
    }

    pub fn begin(&mut self, label: wgpu::Label) {
        self.encoder = Some(self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label,
        }));
    }

    pub fn end(&mut self) {
        match self.encoder.take() {
            Some(encoder) => self.queue.submit(Some(encoder.finish())),
            None => panic!("Missing encoder. Command should start() before end()."),
        };
    }

    pub fn present(self) {
        self.surface_texture.present();
    }

    pub fn clear<'d, C: Into<Color<f32>>, U: bytemuck::Zeroable + bytemuck::Pod + bytemuck::NoUninit>(&mut self, color: C, shader: &'d crate::rendering::shaders::Shader, uniforms: Option<&U>) -> Result<(), super::RenderBackendOperationError> {
        self.begin(None);

        let t = Tri::new(Vector2::new(0.0, 0.0), Vector2::new(10.0, 10.0),  Vector2::new(10.0, 0.0));

        let draw_config = DrawConfig {
            position: Vector2::new(40.0, 40.0),
        };

        t.draw(self, &draw_config)
         .using_shader::<()>(shader, None)
         .submit()
         .unwrap();

        /*
        if let Some(ref mut encoder) = &mut self.encoder {
            let pass = RenderPass::new(
                encoder,
                &self.surface_view,
                &self.device,
                &self.shader_builder,
                vec!(t.a, t.b, t.c),
                &DrawConfig::EMPTY,
            );

            pass.clear_color(color)
                .using_shader::<()>(shader, None)
                .submit()?;
        }

        */

        self.end();

        Ok(())
    }

    pub fn draw_vertices<'d>(
        &'d mut self,
        vertices: Vec<Vector2<f32>>,
        config: &'d DrawConfig,
    ) -> RenderPass<'d> {
        match &mut self.encoder {
            Some(ref mut encoder) => {
                RenderPass::new(
                    encoder,
                    &self.surface_view,
                    &self.device,
                    &self.shader_builder,
                    vertices,
                    config,
                )
            },
            None => panic!("Missing encoder. Command should start() before drawing something."),
        }
    }
}
