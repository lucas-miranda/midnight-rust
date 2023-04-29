use crate::rendering::shaders::{
    ShaderInstance,
    VertexAttribute,
};

use super::{
    ShaderBuilder,
    ShaderFormat,
};

pub struct ShaderInstanceBuilder<'a, U> {
    builder: &'a mut ShaderBuilder,
    format: ShaderFormat,
    vertex: &'a str,
    fragment: &'a str,
    vertex_attributes: Vec<VertexAttribute>,
    phantom: std::marker::PhantomData<U>,
}

impl<'a, U> ShaderInstanceBuilder<'a, U> {
    pub(super) fn new(
        builder: &'a mut ShaderBuilder,
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
            phantom: Default::default(),
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

    pub fn build<S: ShaderInstance>(self) -> S {
        self.builder.build::<U, S>(
            self.format,
            self.vertex,
            self.fragment,
            self.vertex_attributes,
        )
    }
}
