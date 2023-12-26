mod backends;
pub(super) use backends::*;

mod instance_builder;
pub use instance_builder::ShaderInstanceBuilder;

mod shader_context;
pub(crate) use shader_context::*;

mod processor;
pub(in crate::rendering::shaders) use processor::ShaderProcessor;

mod processor_error;
pub(in crate::rendering::shaders) use processor_error::ShaderProcessorError;

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

use super::{ShaderId, ShaderInstance, ShaderDescriptorError};

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub enum ShaderFormat {
    GLSL,
    HLSL,
    WGSL,
}

pub struct ShaderBuilder {
    device: Weak<wgpu::Device>,
    surface_format: wgpu::TextureFormat,
    next_shader_id: ShaderId,
    backend: ShaderBackend,
    contexts: HashMap<Shader, ShaderContext>,
    instances: HashMap<Shader, Weak<RefCell<dyn ShaderInstance>>>,
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
            backend: ShaderBackend::default(),
            contexts: HashMap::new(),
            instances: HashMap::new(),
            resources: Default::default(),
        }
    }

    pub fn get_context(&self, shader: &Shader) -> Option<&ShaderContext> {
        self.contexts.get(shader)
    }

    pub fn get_mut_context(&mut self, shader: &Shader) -> Option<&mut ShaderContext> {
        self.contexts.get_mut(shader)
    }

    pub fn resources(&self) -> &ShaderResources {
        &self.resources
    }

    pub fn get_instance(&self, shader: &Shader) -> Option<&Weak<RefCell<dyn ShaderInstance>>> {
        self.instances.get(shader)
    }

    pub fn instances(&self) -> &HashMap<Shader, Weak<RefCell<dyn ShaderInstance>>> {
        &self.instances
    }

    pub fn create<'b>(
        &'b mut self,
        descriptor: ShaderDescriptor<'b>,
    ) -> ShaderInstanceBuilder<'b> {
        ShaderInstanceBuilder::new(self, descriptor)
    }

    pub fn destroy(&mut self, shader: Shader) {
        self.contexts.remove(&shader);
    }

    fn next_shader_id(&mut self) -> ShaderId {
        let id = self.next_shader_id;
        self.next_shader_id.next();
        id
    }

    fn build<S: ShaderInstance + 'static>(
        &mut self,
        descriptor: ShaderDescriptor,
        vertex_attributes: Vec<VertexAttribute>,
        bindings: Vec<BindingsDescriptorEntry>,
    ) -> Result<Rc<RefCell<S>>, ShaderDescriptorError> {
        let shader = Shader::new(self.next_shader_id());
        let device = self.device.upgrade().unwrap();

        let context = ShaderContext::new::<_>(
                ShaderProcessor::new(&self.backend),
                &descriptor,
                device,
                self.surface_format,
                vertex_attributes,
                bindings,
            )?;

        self.contexts.insert(shader, context);
        self.resources.insert(shader);

        let instance = {
            let instance = Rc::new(RefCell::new(S::new(shader)));
            let weak = Rc::downgrade(&instance);
            self.instances.insert(shader, weak);

            instance
        };

        Ok(instance)
    }
}

