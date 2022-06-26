#[cfg(feature = "shader-naga")]
pub mod naga_builder;

#[cfg(feature = "shader-naga")]
pub use naga_builder as backend;

#[cfg(feature = "shader-shaderc")]
pub mod shaderc_builder;

#[cfg(feature = "shader-shaderc")]
pub use shaderc_builder as backend;

use crate::rendering::shaders::{Shader, ShaderId};

pub trait ShaderBuilderBackend : Default {
    type GLSL: ShaderGLSLBackendProcessor;

    fn glsl(&self) -> &Self::GLSL;
}

pub trait ShaderGLSLBackendProcessor {
    fn build(&self, id: ShaderId, vertex: &str, fragment: &str) -> Shader;
}
