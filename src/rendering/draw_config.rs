use crate::math::Vector2;

pub use wgpu::{
    Face,
    FrontFace,
    IndexFormat,
    PolygonMode,
    PrimitiveState,
    PrimitiveTopology,
};

use super::shaders::{
    ShaderId,
    ShaderInfo,
};

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DrawConfig {
    pub position: Vector2<f32>,
    pub shader_config: Option<ShaderConfig>,
}

impl DrawConfig {
    pub const EMPTY: Self = DrawConfig {
        position: Vector2::new(0.0, 0.0),
        shader_config: None,
    };
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ShaderConfig {
    shader_id: ShaderId,
    primitive: PrimitiveState,
}

impl ShaderConfig {
    pub fn new<S: ShaderInfo>(shader: &S, primitive: PrimitiveState) -> Self {
        Self {
            shader_id: shader.id(),
            primitive,
        }
    }

    pub(super) fn shader_id(&self) -> &ShaderId {
        &self.shader_id
    }

    pub fn primitive_state(&self) -> &PrimitiveState {
        &self.primitive
    }

    pub fn mut_primitive_state(&mut self) -> &mut PrimitiveState {
        &mut self.primitive
    }
}
