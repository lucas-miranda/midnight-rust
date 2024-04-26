use std::{fmt::Debug, ops::Add};
use bytemuck::{ Pod, Zeroable };
use crate::{
    math::Vector2,
    rendering::{Vertex, VertexPosition, VertexTexture2D},
};

#[repr(C)]
#[derive(Copy, Clone, Default, Debug, Pod, Zeroable)]
pub struct Vertex2DTexture {
    pub position: Vector2<f32>,
    pub uv: Vector2<f32>,
}

impl Vertex for Vertex2DTexture {
}

impl VertexPosition for Vertex2DTexture {
    type Position = Vector2<f32>;

    fn from_position(position: Self::Position) -> Self {
        Self {
            position,
            uv: Vector2::<f32>::default(),
        }
    }

    fn position(&self) -> Self::Position {
        self.position
    }
}

impl VertexTexture2D for Vertex2DTexture {
    fn uv(&self) -> Vector2<f32> {
        self.uv
    }

    fn with_uv(mut self, uv: Vector2<f32>) -> Self {
        self.uv = uv;

        self
    }
}

impl Add<Self> for Vertex2DTexture {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            position: self.position + rhs.position,
            uv: self.uv + rhs.uv,
        }
    }
}
