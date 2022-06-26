use std::{collections::HashMap, rc::Weak};

use wgpu_hal::{
    Api,
    Device,
    ShaderInput,
};

mod backends;
use backends::{
    backend,
    ShaderBuilderBackend,
    ShaderGLSLBackendProcessor,
};

use crate::rendering::{
    shaders::{Shader, ShaderData},
};

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
    pub(crate) fn new(device: Weak<A::Device>, texture_format: wgpu_types::TextureFormat) -> Self {
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

    pub fn build(&mut self, format: ShaderFormat, vertex: &str, fragment: &str) -> Shader {
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

        let shader_desc = wgpu_hal::ShaderModuleDescriptor {
            label: None,
            runtime_checks: false,
        };

        self.contexts.insert(
            shader.id(),
            ShaderContext::new(&shader, device, self.texture_format)
        );

        shader
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
}
