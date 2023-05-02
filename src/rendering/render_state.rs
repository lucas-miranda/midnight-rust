use std::slice::Iter;

use crate::math::Vector2;
use super::DrawConfig;

pub trait RenderState {
    fn extend(&mut self, vertices: Iter<Vector2<f32>>, draw_config: DrawConfig);
}
