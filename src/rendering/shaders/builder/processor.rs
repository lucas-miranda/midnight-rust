use crate::rendering::shaders::{
    ShaderStage,
    ShaderStageDescriptor,
    ShaderStageKind, ShaderRawData,
};

use super::{
    backends::ShaderGLSLBackendProcessor,
    ShaderFormat,
};

pub struct ShaderProcessor<'a> {
    glsl_backend: Option<&'a dyn ShaderGLSLBackendProcessor>,
}

impl<'a> ShaderProcessor<'a> {
    pub fn new(glsl_backend: Option<&'a dyn ShaderGLSLBackendProcessor>) -> Self {
        Self {
            glsl_backend,
        }
    }

    pub fn process(
        &self,
        stage: &ShaderStageKind,
        descriptor: &ShaderStageDescriptor
    ) -> ShaderStage {
        match descriptor.format() {
            ShaderFormat::GLSL => match self.glsl_backend {
                Some(backend) => backend.build((*stage).into(), descriptor.src()),
                None => unimplemented!(),
            },
            ShaderFormat::HLSL => {
                unimplemented!();
            },
            ShaderFormat::WGSL => {
                ShaderStage::new(ShaderRawData::Wgsl(descriptor.src().to_owned()))
            },
        }
    }
}
