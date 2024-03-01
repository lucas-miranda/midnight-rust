use std::{
    ops,
    fmt::{
        self,
        Display,
    },
};

use bytemuck::{Pod, Zeroable};

use crate::math::num_traits::{
    cast::{
        cast,
        NumCast,
    },
    Num,
};

/// Describes a quad-dimensional value.
///
/// It doesn't works as, or shares the same properties with, formal mathematics' vector.
///
/// It's interpretation depends on context, as it can be: a point in space, a direction,
/// a extent or others things.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct Vector4<T> where
    T: Num
{
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T> Vector4<T> where
    T: Num
{
    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        Self {
            x,
            y,
            z,
            w,
        }
    }
}

impl<T> Vector4<T> where
    T: Num + NumCast
{
    pub fn with<U>(x: U, y: U, z: U, w: U) -> Option<Self> where
        U: Num + NumCast
    {
        Some(Self::new(
            cast::<U, T>(x)?,
            cast::<U, T>(y)?,
            cast::<U, T>(z)?,
            cast::<U, T>(w)?,
        ))
    }
}

impl<T: Num + Display> Display for Vector4<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}, {}, {}, {}", self.x, self.y, self.z, self.w)
    }
}

impl<T> ops::Neg for Vector4<T> where
    T: Num + ops::Neg<Output = T>
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl<T> ops::Add for Vector4<T> where
    T: Num
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl<T> ops::Add<T> for Vector4<T> where
    T: Num + Copy
{
    type Output = Self;

    fn add(self, value: T) -> Self::Output {
        Self {
            x: self.x + value,
            y: self.y + value,
            z: self.z + value,
            w: self.w + value,
        }
    }
}

impl<T> ops::Sub for Vector4<T> where
    T: Num
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl<T> ops::Sub<T> for Vector4<T> where
    T: Num + Copy
{
    type Output = Self;

    fn sub(self, value: T) -> Self::Output {
        Self {
            x: self.x - value,
            y: self.y - value,
            z: self.z - value,
            w: self.w - value,
        }
    }
}

impl<T> ops::Mul for Vector4<T> where
    T: Num
{
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            w: self.w * other.w,
        }
    }
}

impl<T> ops::Mul<T> for Vector4<T> where
    T: Num + Copy
{
    type Output = Self;

    fn mul(self, value: T) -> Self::Output {
        Self {
            x: self.x * value,
            y: self.y * value,
            z: self.z * value,
            w: self.w * value,
        }
    }
}

impl<T> ops::Div for Vector4<T> where
    T: Num
{
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
            w: self.w / other.w,
        }
    }
}

impl<T> ops::Div<T> for Vector4<T> where
    T: Num + Copy
{
    type Output = Self;

    fn div(self, value: T) -> Self::Output {
        Self {
            x: self.x / value,
            y: self.y / value,
            z: self.z / value,
            w: self.w / value,
        }
    }
}

impl<T> ops::Rem for Vector4<T> where
    T: Num
{
    type Output = Self;

    fn rem(self, other: Self) -> Self::Output {
        Self {
            x: self.x % other.x,
            y: self.y % other.y,
            z: self.z % other.z,
            w: self.w % other.w,
        }
    }
}

impl<T> ops::Rem<T> for Vector4<T> where
    T: Num + Copy
{
    type Output = Self;

    fn rem(self, value: T) -> Self::Output {
        Self {
            x: self.x % value,
            y: self.y % value,
            z: self.z % value,
            w: self.w % value,
        }
    }
}

impl<T> ops::AddAssign for Vector4<T> where
    T: Num + Copy
{
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl<T> ops::AddAssign<T> for Vector4<T> where
    T: Num + Copy
{
    fn add_assign(&mut self, value: T) {
        *self = Self {
            x: self.x + value,
            y: self.y + value,
            z: self.z + value,
            w: self.w + value,
        }
    }
}

impl<T> ops::SubAssign for Vector4<T> where
    T: Num + Copy
{
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl<T> ops::SubAssign<T> for Vector4<T> where
    T: Num + Copy
{
    fn sub_assign(&mut self, value: T) {
        *self = Self {
            x: self.x - value,
            y: self.y - value,
            z: self.z - value,
            w: self.w - value,
        }
    }
}

impl<T> ops::MulAssign for Vector4<T> where
    T: Num + Copy
{
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            w: self.w * other.w,
        }
    }
}

impl<T> ops::MulAssign<T> for Vector4<T> where
    T: Num + Copy
{
    fn mul_assign(&mut self, value: T) {
        *self = Self {
            x: self.x * value,
            y: self.y * value,
            z: self.z * value,
            w: self.w * value,
        }
    }
}

impl<T> ops::DivAssign for Vector4<T> where
    T: Num + Copy
{
    fn div_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
            w: self.w / other.w,
        }
    }
}

impl<T> ops::DivAssign<T> for Vector4<T> where
    T: Num + Copy
{
    fn div_assign(&mut self, value: T) {
        *self = Self {
            x: self.x / value,
            y: self.y / value,
            z: self.z / value,
            w: self.w / value,
        }
    }
}

impl<T> ops::RemAssign for Vector4<T> where
    T: Num + Copy
{
    fn rem_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x % other.x,
            y: self.y % other.y,
            z: self.z % other.z,
            w: self.w % other.w,
        }
    }
}

impl<T> ops::RemAssign<T> for Vector4<T> where
    T: Num + Copy
{
    fn rem_assign(&mut self, value: T) {
        *self = Self {
            x: self.x % value,
            y: self.y % value,
            z: self.z % value,
            w: self.w % value,
        }
    }
}

impl<T> ops::Index<usize> for Vector4<T> where
    T: Num
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("Index was out of range, it must be in range [0, 3]")
        }
    }
}

impl<T> ops::IndexMut<usize> for Vector4<T> where
    T: Num
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("Index was out of range, it must be in range [0, 3]")
        }
    }
}

impl<T> From<(T, T, T, T)> for Vector4<T> where
    T: Num + Copy
{
    fn from(tuple: (T, T, T, T)) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
            z: tuple.2,
            w: tuple.3,
        }
    }
}

impl<T> From<&(T, T, T, T)> for Vector4<T> where
    T: Num + Copy
{
    fn from(tuple: &(T, T, T, T)) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
            z: tuple.2,
            w: tuple.3,
        }
    }
}

impl<T> From<[T; 4]> for Vector4<T> where
    T: Num + Copy
{
    fn from(slice: [T; 4]) -> Self {
        Self {
            x: slice[0],
            y: slice[1],
            z: slice[2],
            w: slice[3],
        }
    }
}

impl<T> From<&[T; 4]> for Vector4<T> where
    T: Num + Copy
{
    fn from(slice: &[T; 4]) -> Self {
        Self {
            x: slice[0],
            y: slice[1],
            z: slice[2],
            w: slice[3],
        }
    }
}

impl<T> From<Vector4<T>> for [T; 4] where
    T: Num + Copy
{
    fn from(vec4: Vector4<T>) -> Self {
        [ vec4.x, vec4.y, vec4.z, vec4.w ]
    }
}

unsafe impl Zeroable for Vector4<f32> {
}

unsafe impl Pod for Vector4<f32> {
}
