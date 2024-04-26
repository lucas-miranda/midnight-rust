use std::{fmt::Debug, ops::Add};
use bytemuck::{Pod, Zeroable};
use crate::{
    math::Vector2,
    rendering::{Vertex, VertexPosition, VertexTexture2D, Color, VertexColor},
};

#[repr(C)]
#[derive(Copy, Clone, Default, Debug, Pod, Zeroable)]
pub struct Vertex2DTextureColor {
    pub position: Vector2<f32>,
    pub uv: Vector2<f32>,
    pub color: Color<f32>,
}

impl Vertex for Vertex2DTextureColor {
}

impl VertexPosition for Vertex2DTextureColor {
    type Position = Vector2<f32>;

    fn from_position(position: Self::Position) -> Self {
        Self {
            position,
            uv: Vector2::<f32>::default(),
            color: Color::<f32>::TRANSPARENT_BLACK,
        }
    }

    fn position(&self) -> Self::Position {
        self.position
    }
}

impl VertexTexture2D for Vertex2DTextureColor {
    fn uv(&self) -> Vector2<f32> {
        self.uv
    }

    fn with_uv(mut self, uv: Vector2<f32>) -> Self {
        self.uv = uv;

        self
    }
}

impl VertexColor for Vertex2DTextureColor {
    fn color(&self) -> Color<f32> {
        self.color
    }

    fn with_color(mut self, color: Color<f32>) -> Self {
        self.color = color;

        self
    }
}

impl Add<Self> for Vertex2DTextureColor {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            position: self.position + rhs.position,
            uv: self.uv + rhs.uv,
            color: self.color + rhs.color,
        }
    }
}
