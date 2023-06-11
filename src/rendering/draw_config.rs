use crate::math::Vector2;

pub use wgpu::{
    Face,
    FrontFace,
    IndexFormat,
    PolygonMode,
    PrimitiveState,
    PrimitiveTopology,
};

use super::{
    shaders::{
        ShaderId,
        ShaderInfo,
    },
    Vertex,
};

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DrawConfig<V: Vertex> {
    pub vertex: V,
    pub shader_config: Option<ShaderConfig>,
}

/*
impl<V: Vertex> DrawConfig<V> {
    pub const EMPTY: Self = DrawConfig {
        vertex: V::default(),
        shader_config: None,
    };
}
*/

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
