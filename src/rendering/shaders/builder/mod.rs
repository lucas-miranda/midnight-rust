use std::{
    collections::HashMap,
    rc::Weak,
};

mod backends;
use backends::{
    backend,
    ShaderBuilderBackend,
    ShaderGLSLBackendProcessor,
};

mod instance_builder;
pub use instance_builder::ShaderInstanceBuilder;

pub use wgpu::PrimitiveTopology;

use crate::rendering::shaders::{
    Shader,
    VertexAttribute,
};

use super::{ShaderId, ShaderInstance};

mod shader_context;
pub(crate) use shader_context::*;

pub type ShaderGLSLProcessor = <backend::Backend as ShaderBuilderBackend>::GLSL;

#[non_exhaustive]
#[derive(Debug)]
pub enum ShaderFormat {
    GLSL,
    HLSL,
}

pub struct ShaderBuilder {
    device: Weak<wgpu::Device>,
    surface_format: wgpu::TextureFormat,
    next_shader_id: ShaderId,
    backend: backend::Backend,
    contexts: HashMap<ShaderId, ShaderContext>,
}

impl ShaderBuilder {
    pub(crate) fn new(
        device: Weak<wgpu::Device>,
        surface_format: wgpu::TextureFormat,
    ) -> Self {
        Self {
            device,
            surface_format,
            next_shader_id: ShaderId::default(),
            backend: backend::Backend::default(),
            contexts: HashMap::new(),
        }
    }

    pub fn get_context(&self, shader_id: &ShaderId) -> Option<&ShaderContext> {
        self.contexts.get(shader_id)
    }

    pub fn get_mut_context(&mut self, shader_id: &ShaderId) -> Option<&mut ShaderContext> {
        self.contexts.get_mut(shader_id)
    }

    pub fn create<'b, U>(
        &'b mut self,
        format: ShaderFormat,
        vertex: &'b str,
        fragment: &'b str,
    ) -> ShaderInstanceBuilder<'b, U> {
        ShaderInstanceBuilder::new(self, format, vertex, fragment)
    }

    pub fn destroy(&mut self, shader: Shader) {
        self.contexts.remove(shader.id());
    }

    fn glsl(&self) -> &ShaderGLSLProcessor {
        &self.backend.glsl()
    }

    fn next_shader_id(&mut self) -> ShaderId {
        let id = self.next_shader_id;
        self.next_shader_id.next();
        id
    }

    fn build<U, S: ShaderInstance>(
        &mut self,
        format: ShaderFormat,
        vertex: &str,
        fragment: &str,
        vertex_attributes: Vec<VertexAttribute>,
    ) -> S {
        let id = self.next_shader_id();

        let shader = match format {
            ShaderFormat::GLSL => {
                self.glsl().build(id, vertex, fragment)
            },
            ShaderFormat::HLSL => {
                unimplemented!();
            },
        };

        let device = self.device.upgrade().unwrap();

        self.contexts.insert(
            *shader.id(),
            ShaderContext::new::<_, U>(
                &shader,
                device,
                self.surface_format,
                vertex_attributes,
            )
        );

        S::new(shader)
    }
}

