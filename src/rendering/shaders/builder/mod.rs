mod backends;
use backends::{
    backend,
    ShaderBuilderBackend,
    ShaderGLSLBackendProcessor,
};

mod instance_builder;
pub use instance_builder::ShaderInstanceBuilder;

mod shader_context;
pub(crate) use shader_context::*;

mod processor;
pub(in crate::rendering::shaders) use processor::ShaderProcessor;

pub use wgpu::PrimitiveTopology;

use std::{
    collections::HashMap,
    rc::{ Rc, Weak },
    cell::RefCell,
};

use crate::{
    rendering::shaders::{
        BindingsDescriptorEntry,
        Shader,
        ShaderDescriptor,
        VertexAttribute,
    },
    resources::ShaderResources,
};

use super::{ShaderId, ShaderInstance};

pub type ShaderGLSLProcessor = <backend::Backend as ShaderBuilderBackend>::GLSL;

#[non_exhaustive]
#[derive(Debug)]
pub enum ShaderFormat {
    GLSL,
    HLSL,
    WGSL,
}

pub struct ShaderBuilder {
    device: Weak<wgpu::Device>,
    surface_format: wgpu::TextureFormat,
    next_shader_id: ShaderId,
    backend: backend::Backend,
    contexts: HashMap<ShaderId, ShaderContext>,
    instances: HashMap<ShaderId, Weak<RefCell<dyn ShaderInstance>>>,
    resources: ShaderResources,
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
            instances: HashMap::new(),
            resources: Default::default(),
        }
    }

    pub fn get_context(&self, shader_id: &ShaderId) -> Option<&ShaderContext> {
        self.contexts.get(shader_id)
    }

    pub fn get_mut_context(&mut self, shader_id: &ShaderId) -> Option<&mut ShaderContext> {
        self.contexts.get_mut(shader_id)
    }

    pub fn resources(&self) -> &ShaderResources {
        &self.resources
    }

    pub fn instances(&self) -> &HashMap<ShaderId, Weak<RefCell<dyn ShaderInstance>>> {
        &self.instances
    }

    pub fn create<'b, U>(
        &'b mut self,
        descriptor: ShaderDescriptor<'b>,
    ) -> ShaderInstanceBuilder<'b, U> {
        ShaderInstanceBuilder::new(self, descriptor)
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

    fn build<U, S: ShaderInstance + 'static>(
        &mut self,
        descriptor: ShaderDescriptor,
        vertex_attributes: Vec<VertexAttribute>,
        bindings: Vec<BindingsDescriptorEntry<U>>,
    ) -> Rc<RefCell<S>> {
        let id = self.next_shader_id();
        let device = self.device.upgrade().unwrap();

        self.contexts.insert(
            id,
            ShaderContext::new::<_, U>(
                ShaderProcessor::new(Some(&self.backend)),
                &descriptor,
                device,
                self.surface_format,
                vertex_attributes,
                bindings,
            )
        );

        self.resources.insert(id);

        let instance = {
            let shader = Shader::new(id);
            let instance = Rc::new(RefCell::new(S::new(shader)));
            let weak = Rc::downgrade(&instance);
            self.instances.insert(id, weak);

            instance
        };

        instance
    }
}

