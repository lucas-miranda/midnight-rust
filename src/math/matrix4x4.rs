use std::fmt::{
    self,
    Display,
};

use bytemuck::{Pod, Zeroable};

use crate::math::num_traits::{
    cast::NumCast,
    Num,
};

use super::geometry::Vector4;

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct Matrix4x4<T> where T : Num {
    pub row_0: Vector4<T>,
    pub row_1: Vector4<T>,
    pub row_2: Vector4<T>,
    pub row_3: Vector4<T>,
}

impl<T> Matrix4x4<T> where
    T: Num + NumCast
{
    pub const fn with_rows(row_0: Vector4<T>, row_1: Vector4<T>, row_2: Vector4<T>, row_3: Vector4<T>) -> Self {
        Self {
            row_0,
            row_1,
            row_2,
            row_3,
        }
    }
}

impl Matrix4x4<f32> {
    pub fn ortho(b: f32, t: f32, l: f32, r: f32, n: f32, f: f32) -> Self {
        Self::with_rows(
            Vector4::new(2.0 / (r - l), 0.0,            0.0,            - (r + l) / (r - l)),
            Vector4::new(0.0,           2.0 / (t - b),  0.0,            - (t + b) / (t - b)),
            Vector4::new(0.0,           0.0,            -2.0 / (f - n), - (f + n) / (f - n)),
            Vector4::new(0.0,           0.0,            0.0,            1.0),
        )
    }
}

impl<T: Num + Display> Display for Matrix4x4<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "[ {}  {}  {}  {} ][ {}  {}  {}  {} ][ {}  {}  {}  {} ][ {}  {}  {}  {} ]",
            self.row_0.x, self.row_0.y, self.row_0.z, self.row_0.w,
            self.row_1.x, self.row_1.y, self.row_1.z, self.row_1.w,
            self.row_2.x, self.row_2.y, self.row_2.z, self.row_2.w,
            self.row_3.x, self.row_3.y, self.row_3.z, self.row_3.w,
        )
    }
}

impl<T> From<Matrix4x4<T>> for [T; 16] where
    T: Num + Copy
{
    fn from(mat4x4: Matrix4x4<T>) -> Self {
        [
            mat4x4.row_0.x, mat4x4.row_0.y, mat4x4.row_0.z, mat4x4.row_0.w,
            mat4x4.row_1.x, mat4x4.row_1.y, mat4x4.row_1.z, mat4x4.row_1.w,
            mat4x4.row_2.x, mat4x4.row_2.y, mat4x4.row_2.z, mat4x4.row_2.w,
            mat4x4.row_3.x, mat4x4.row_3.y, mat4x4.row_3.z, mat4x4.row_3.w,
        ]
    }
}

unsafe impl Zeroable for Matrix4x4<f32> {
}

unsafe impl Pod for Matrix4x4<f32> {
}
