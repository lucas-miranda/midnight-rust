#[cfg(feature = "shader-naga")]
pub mod naga_builder;
#[cfg(feature = "shader-naga")]
pub use naga_builder as backend;

#[cfg(feature = "shader-shaderc")]
pub mod shaderc_builder;
#[cfg(feature = "shader-shaderc")]
pub use shaderc_builder::ShadercBuilderBackend;

pub mod error;
pub use error::ShaderBackendError;

pub mod empty_builder;
pub use empty_builder::EmptyBuilderBackend;

use crate::rendering::shaders::{
    ShaderStageData,
    ShaderStageKind,
};

// TODO
//  - specify an entry point
//  - compile one file into multiple shader stages

pub trait GLSLShaderProcessor {
    fn build(&self, stage: ShaderStageKind, src: &str) -> Result<ShaderStageData, ShaderBackendError>;
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub enum ShaderBackendKind {
    None,

    #[cfg(feature = "shader-shaderc")]
    Shaderc,

    #[cfg(feature = "shader-naga")]
    Naga,
}

#[derive(Default)]
pub struct ShaderBackend {
    #[cfg(feature = "shader-shaderc")]
    shaderc: ShadercBuilderBackend,
}

impl ShaderBackend {
    /// Returns first available glsl backend.
    pub fn glsl(&self) -> Option<&impl GLSLShaderProcessor> {
        #[cfg(feature = "shader-shaderc")]
        return Some(&self.shaderc);

        #[cfg(all(
            feature = "shader-naga",
            not(feature = "shader-shaderc")
        ))]
        return Some(&self.naga);

        #[cfg(not(any(
            feature = "shader-shaderc",
            feature = "shader-naga"
        )))]
        return None::<&EmptyBuilderBackend>;
    }
}
