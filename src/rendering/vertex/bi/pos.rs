use std::{
    fmt::Debug,
    ops::Add,
};

use bytemuck::{ Pod, Zeroable };
use crate::{
    math::Vector2,
    rendering::{Vertex, VertexPosition},
};

#[repr(C)]
#[derive(Copy, Clone, Default, Debug, Pod, Zeroable)]
pub struct Vertex2D {
    pub position: Vector2<f32>
}

impl Vertex for Vertex2D {
}

impl VertexPosition for Vertex2D {
    type Position = Vector2<f32>;

    fn from_position(position: Self::Position) -> Self {
        Self {
            position,
        }
    }

    fn position(&self) -> Self::Position {
        self.position
    }
}

impl Add<Self> for Vertex2D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            position: self.position + rhs.position,
        }
    }
}
