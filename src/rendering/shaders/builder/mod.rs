use std::{collections::HashMap, iter, rc::Weak};

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
pub(crate) use shader_context::ShaderContext;

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

    pub fn create<'a, U>(
        &'a mut self,
        format: ShaderFormat,
        vertex: &'a str,
        fragment: &'a str,
        primitive_topology: PrimitiveTopology,
    ) -> ShaderInstanceBuilder<'a, U> {
        ShaderInstanceBuilder::new(self, format, vertex, fragment, primitive_topology)
    }

    pub fn destroy(&mut self, shader: Shader) {
        self.contexts.remove(&shader.id());
    }

    fn glsl(&self) -> &ShaderGLSLProcessor {
        &self.backend.glsl()
    }

    fn next_shader_id(&mut self) -> ShaderId {
        let id = self.next_shader_id;
        self.next_shader_id += 1;
        id
    }

    fn build<U, S: ShaderInstance>(
        &mut self,
        format: ShaderFormat,
        vertex: &str,
        fragment: &str,
        vertex_attributes: Vec<VertexAttribute>,
        primitive_topology: PrimitiveTopology,
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
            shader.id(),
            ShaderContext::new::<_, U>(
                &shader,
                device,
                self.surface_format,
                iter::once(vertex_attributes)
                    .map(|attrs| attrs
                        .into_iter()
                        .map(wgpu::VertexAttribute::from)
                        .collect::<Vec<wgpu::VertexAttribute>>()
                    )
                    .collect::<Vec<Vec<_>>>()
                    .as_slice(),
                primitive_topology,
            )
        );

        S::new(shader)
    }
}

