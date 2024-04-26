use std::{fmt::Debug, ops::Add};
use bytemuck::{Pod, Zeroable};
use crate::{
    math::Vector2,
    rendering::{Vertex, VertexPosition, Color, VertexColor},
};

#[repr(C)]
#[derive(Copy, Clone, Default, Debug, Pod, Zeroable)]
pub struct Vertex2DColor {
    pub position: Vector2<f32>,
    pub color: Color<f32>,
}

impl Vertex for Vertex2DColor {
}

impl VertexPosition for Vertex2DColor {
    type Position = Vector2<f32>;

    fn from_position(position: Self::Position) -> Self {
        Self {
            position,
            color: Color::<f32>::WHITE,
        }
    }

    fn position(&self) -> Self::Position {
        self.position
    }
}

impl VertexColor for Vertex2DColor {
    fn color(&self) -> Color<f32> {
        self.color
    }

    fn with_color(mut self, color: Color<f32>) -> Self {
        self.color = color;

        self
    }
}

impl Add<Self> for Vertex2DColor {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            position: self.position + rhs.position,
            color: self.color + rhs.color,
        }
    }
}
