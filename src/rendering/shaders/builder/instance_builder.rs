use std::{
    cell::RefCell,
    rc::Rc,
};

use crate::rendering::shaders::{
    BindingsDescriptorEntry,
    ShaderInstance,
    VertexAttribute,
    ShaderDescriptor,
    ShaderDescriptorError,
};

use super::ShaderBuilder;

pub struct ShaderInstanceBuilder<'a> {
    builder: &'a mut ShaderBuilder,
    descriptor: ShaderDescriptor<'a>,
    vertex_attributes: Vec<VertexAttribute>,
    bindings: Vec<BindingsDescriptorEntry>,
}

impl<'a> ShaderInstanceBuilder<'a> {
    pub(super) fn new(
        builder: &'a mut ShaderBuilder,
        descriptor: ShaderDescriptor<'a>,
    ) -> Self {
        Self {
            builder,
            descriptor,
            vertex_attributes: Vec::new(),
            //phantom: Default::default(),
            bindings: Vec::new(),
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

    pub fn bindings<I>(mut self, descriptor: I) -> Self where
        I: Iterator<Item = BindingsDescriptorEntry>
    {
        self.bindings.clear();

        for entry in descriptor {
            self.bindings.push(entry);
        }

        self
    }

    pub fn build<S: ShaderInstance + 'static>(self) -> Result<Rc<RefCell<S>>, ShaderDescriptorError> {
        self.builder.build::<S>(
                self.descriptor,
                self.vertex_attributes,
                self.bindings,
            )
    }
}
