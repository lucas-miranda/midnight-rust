use crate::rendering::shaders::{
    stage::ShaderStageData,
    ShaderRawData,
    ShaderStageDescriptor,
    ShaderStageKind,
};

use super::{
    ShaderBackend,
    ShaderFormat,
    ShaderProcessorError,
};

#[cfg(any(
    feature = "shader-shaderc",
    feature = "shader-naga"
))]
use super::GLSLShaderProcessor;

#[cfg(any(
    feature = "shader-shaderc",
    feature = "shader-naga"
))]
pub struct ShaderProcessor<'a> {
    backend: &'a ShaderBackend,
}

#[cfg(not(any(
    feature = "shader-shaderc",
    feature = "shader-naga"
)))]
pub struct ShaderProcessor<'a> {
    phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a> ShaderProcessor<'a> {
    #[cfg(any(
        feature = "shader-shaderc",
        feature = "shader-naga"
    ))]
    pub fn new(backend: &'a ShaderBackend) -> Self {
        Self {
            backend,
        }
    }

    #[cfg(not(any(
        feature = "shader-shaderc",
        feature = "shader-naga"
    )))]
    pub fn new(_backend: &'a ShaderBackend) -> Self {
        Self {
            phantom: Default::default(),
        }
    }

    pub fn process(
        &self,
        _stage: &ShaderStageKind,
        descriptor: &ShaderStageDescriptor
    ) -> Result<ShaderStageData, ShaderProcessorError> {
        match descriptor.format() {
            ShaderFormat::WGSL => Ok(ShaderStageData::new(
                ShaderRawData::Wgsl(descriptor.src().to_owned())
            )),

            #[cfg(any(
                feature = "shader-shaderc",
                feature = "shader-naga"
            ))]
            ShaderFormat::GLSL => match self.backend.glsl() {
                Some(backend) => backend.build((*stage).into(), descriptor.src()),
                None => unimplemented!(),
            },

            #[cfg(any(
                feature = "shader-shaderc",
                feature = "shader-naga"
            ))]
            ShaderFormat::HLSL => {
                unimplemented!();
            },

            #[cfg(not(any(
                feature = "shader-shaderc",
                feature = "shader-naga"
            )))]
            _ => Err(ShaderProcessorError::FormatNotSupported(*descriptor.format())),
        }
    }
}
