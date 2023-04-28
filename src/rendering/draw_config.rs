use crate::math::Vector2;

#[derive(Debug, Copy, Clone, Default)]
pub struct DrawConfig {
    pub position: Vector2<f32>,
}

impl DrawConfig {
    pub const EMPTY: Self = DrawConfig {
        position: Vector2::new(0.0, 0.0),
    };
}

