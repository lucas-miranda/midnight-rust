mod backends;
use backends::{
    backend,
    ShaderBuilderBackend,
    ShaderGLSLBackendProcessor,
};

use crate::rendering::shaders::Shader;

pub type ShaderGLSLProcessor = <backend::Backend as ShaderBuilderBackend>::GLSL;

#[non_exhaustive]
#[derive(Debug)]
pub enum ShaderFormat {
    GLSL,
    HLSL,
}

#[derive(Default)]
pub struct ShaderBuilder {
    backend: backend::Backend,
}

impl ShaderBuilder {
    pub fn build(&self, format: ShaderFormat, vertex: &str, fragment: &str) -> Shader {
        match format {
            ShaderFormat::GLSL => {
                self.glsl().build(vertex, fragment)
            },
            ShaderFormat::HLSL => {
                unimplemented!();
            },
        }
    }

    fn glsl(&self) -> &ShaderGLSLProcessor {
        &self.backend.glsl()
    }
}
