#[allow(unused_imports)]
pub use wgpu::{
    Face,
    FrontFace,
    IndexFormat,
    PolygonMode,
    PrimitiveState,
    PrimitiveTopology,
};

use crate::rendering::shaders::{Shader, ShaderInfo};

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ShaderConfig {
    shader: Shader,
    primitive: PrimitiveState,
}

impl ShaderConfig {
    pub fn new<S: ShaderInfo>(shader: &S, primitive: PrimitiveState) -> Self {
        Self {
            shader: shader.identifier(),
            primitive,
        }
    }

    pub(crate) fn shader(&self) -> &Shader {
        &self.shader
    }

    pub fn primitive_state(&self) -> &PrimitiveState {
        &self.primitive
    }

    pub fn mut_primitive_state(&mut self) -> &mut PrimitiveState {
        &mut self.primitive
    }
}
