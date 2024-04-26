mod bi;
pub use bi::*;

use std::fmt::Debug;

use bytemuck::{ Pod, Zeroable };
use crate::math::Vector2;

use super::Color;

/// Base vertex.
pub trait Vertex
    : Pod
        + Zeroable
        + Default
        + Debug
        + std::ops::Add<Output = Self>
{
}

/// A vertex which has position.
pub trait VertexPosition : Vertex {
    type Position;

    fn from_position(pos: Self::Position) -> Self;
    fn position(&self) -> Self::Position;
}

/// A vertex which has a 2d uv.
pub trait VertexTexture2D : Vertex {
    fn uv(&self) -> Vector2<f32>;
    fn with_uv(self, uv: Vector2<f32>) -> Self;
}

/// A vertex which has color.
pub trait VertexColor : Vertex {
    fn color(&self) -> Color<f32>;
    fn with_color(self, color: Color<f32>) -> Self;
}

