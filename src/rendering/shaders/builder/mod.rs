use std::{collections::HashMap, iter, rc::Weak};

use wgpu_hal::{
    Api,
    Device,
};

mod backends;
use backends::{
    backend,
    ShaderBuilderBackend,
    ShaderGLSLBackendProcessor,
};

use crate::rendering::shaders::{Shader, VertexAttribute};

use super::ShaderId;

mod shader_context;
pub(crate) use shader_context::ShaderContext;

pub type ShaderGLSLProcessor = <backend::Backend as ShaderBuilderBackend>::GLSL;

#[non_exhaustive]
#[derive(Debug)]
pub enum ShaderFormat {
    GLSL,
    HLSL,
}

pub struct ShaderBuilder<A: Api> {
    device: Weak<A::Device>,
    texture_format: wgpu_types::TextureFormat,
    next_shader_id: ShaderId,
    backend: backend::Backend,
    contexts: HashMap<ShaderId, ShaderContext<A>>,
}

impl<A: Api> ShaderBuilder<A> {
    pub(crate) fn new(
        device: Weak<A::Device>,
        texture_format: wgpu_types::TextureFormat,
    ) -> Self {
        Self {
            device,
            texture_format,
            next_shader_id: ShaderId::default(),
            backend: backend::Backend::default(),
            contexts: HashMap::new(),
        }
    }

    pub fn get_context(&self, shader_id: &ShaderId) -> Option<&ShaderContext<A>> {
        self.contexts.get(shader_id)
    }

    pub fn create<'a>(
        &'a mut self,
        format: ShaderFormat,
        vertex: &'a str,
        fragment: &'a str,
    ) -> ShaderInstanceBuilder<'a, A> {
        ShaderInstanceBuilder::new(self, format, vertex, fragment)
    }

    pub fn destroy(&mut self, shader: Shader) {
        if let Some(context) = self.contexts.remove(&shader.id()) {
            let device = self.device.upgrade().unwrap();

            unsafe {
                device.destroy_render_pipeline(context.pipeline);
                device.destroy_pipeline_layout(context.pipeline_layout);
                device.destroy_shader_module(context.vertex_module);
                device.destroy_shader_module(context.fragment_module);
            }
        }
    }

    fn glsl(&self) -> &ShaderGLSLProcessor {
        &self.backend.glsl()
    }

    fn next_shader_id(&mut self) -> ShaderId {
        let id = self.next_shader_id;
        self.next_shader_id += 1;
        id
    }

    fn build(
        &mut self,
        format: ShaderFormat,
        vertex: &str,
        fragment: &str,
        vertex_attributes: Vec<VertexAttribute>,
    ) -> Shader {
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
            ShaderContext::new(
                &shader,
                device,
                self.texture_format,
                iter::once(vertex_attributes)
                    .map(|attrs| attrs
                        .into_iter()
                        .map(wgpu_types::VertexAttribute::from)
                        .collect::<Vec<wgpu_types::VertexAttribute>>()
                    )
                    .collect::<Vec<Vec<_>>>()
                    .as_slice(),
            )
        );

        shader
    }
}

pub struct ShaderInstanceBuilder<'a, A: Api> {
    builder: &'a mut ShaderBuilder<A>,
    format: ShaderFormat,
    vertex: &'a str,
    fragment: &'a str,
    vertex_attributes: Vec<VertexAttribute>,
}

impl<'a, A: Api> ShaderInstanceBuilder<'a, A> {
    pub(super) fn new(
        builder: &'a mut ShaderBuilder<A>,
        format: ShaderFormat,
        vertex: &'a str,
        fragment: &'a str,
    ) -> Self {
        Self {
            builder,
            format,
            vertex,
            fragment,
            vertex_attributes: Vec::new(),
        }
    }

    pub fn set_vertex_attributes<I>(mut self, attributes: I) -> Self where
        I: Iterator<Item = VertexAttribute>
    {
        self.vertex_attributes.clear();

        for attribute in attributes {
            self.vertex_attributes.push(attribute);
        }

        self
    }

    pub fn build(self) -> Shader {
        self.builder.build(
            self.format,
            self.vertex,
            self.fragment,
            self.vertex_attributes,
        )
    }
}
