use bytemuck::{ Pod, Zeroable };
use crate::math::Vector2;

pub trait Vertex : Pod + Zeroable + Default + std::ops::Add<Output = Self> {
}

pub trait VertexPosition : Vertex {
    type Position;

    fn from_position(pos: Self::Position) -> Self;
    fn position(&self) -> Self::Position;
}

pub trait VertexTexture2D : Vertex {
    fn uv(&self) -> Vector2<f32>;
    fn with_uv(self, uv: Vector2<f32>) -> Self;
}

//

#[repr(C)]
#[derive(Copy, Clone, Default, Pod, Zeroable)]
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

impl std::ops::Add<Self> for Vertex2D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            position: self.position + rhs.position,
        }
    }
}

//

#[repr(C)]
#[derive(Copy, Clone, Default, Pod, Zeroable)]
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

impl std::ops::Add<Self> for Vertex2DTexture {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            position: self.position + rhs.position,
            uv: self.uv + rhs.uv,
        }
    }
}
