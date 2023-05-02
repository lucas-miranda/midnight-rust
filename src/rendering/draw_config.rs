use crate::math::Vector2;
use super::shaders::ShaderId;

#[derive(Debug, Copy, Clone, Default)]
pub struct DrawConfig {
    pub position: Vector2<f32>,
    pub shader_id: ShaderId,
}

impl DrawConfig {
    pub const EMPTY: Self = DrawConfig {
        position: Vector2::new(0.0, 0.0),
        shader_id: 0,
    };
}

