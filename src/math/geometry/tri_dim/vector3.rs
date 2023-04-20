use std::{
    ops,
    fmt::{
        self,
        Display,
    },
};

use crate::math::num_traits::{
    cast::{
        cast,
        NumCast,
    },
    Num,
};

/// Describes a tri-dimensional value.
/// Shorthand to Vector3<T>.
pub type Vec3<T> = Vector3<T>;

/// Describes a tri-dimensional value.
///
/// It doesn't works as, or shares the same properties with, formal mathematics' vector.
///
/// It's interpretation depends on context, as it can be: a point in space, a direction,
/// a extent or others things.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
#[repr(C)]
pub struct Vector3<T> where
    T: Num
{
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector3<T> where
    T: Num
{
    #[inline]
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self {
            x,
            y,
            z,
        }
    }
}

impl<T> Vector3<T> where
    T: Num + NumCast
{
    pub fn with<U>(x: U, y: U, z: U) -> Option<Self> where
        U: Num + NumCast
    {
        Some(Self::new(
            cast::<U, T>(x)?,
            cast::<U, T>(y)?,
            cast::<U, T>(z)?,
        ))
    }
}

impl<T: Num + Display> Display for Vector3<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}, {}, {}", self.x, self.y, self.z)
    }
}

impl<T> ops::Neg for Vector3<T> where
    T: Num + ops::Neg<Output = T>
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T> ops::Add for Vector3<T> where
    T: Num
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T> ops::Add<T> for Vector3<T> where
    T: Num + Copy
{
    type Output = Self;

    fn add(self, value: T) -> Self::Output {
        Self {
            x: self.x + value,
            y: self.y + value,
            z: self.z + value,
        }
    }
}

impl<T> ops::Sub for Vector3<T> where
    T: Num
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T> ops::Sub<T> for Vector3<T> where
    T: Num + Copy
{
    type Output = Self;

    fn sub(self, value: T) -> Self::Output {
        Self {
            x: self.x - value,
            y: self.y - value,
            z: self.z - value,
        }
    }
}

impl<T> ops::Mul for Vector3<T> where
    T: Num
{
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl<T> ops::Mul<T> for Vector3<T> where
    T: Num + Copy
{
    type Output = Self;

    fn mul(self, value: T) -> Self::Output {
        Self {
            x: self.x * value,
            y: self.y * value,
            z: self.z * value,
        }
    }
}

impl<T> ops::Div for Vector3<T> where
    T: Num
{
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl<T> ops::Div<T> for Vector3<T> where
    T: Num + Copy
{
    type Output = Self;

    fn div(self, value: T) -> Self::Output {
        Self {
            x: self.x / value,
            y: self.y / value,
            z: self.z / value,
        }
    }
}

impl<T> ops::Rem for Vector3<T> where
    T: Num
{
    type Output = Self;

    fn rem(self, other: Self) -> Self::Output {
        Self {
            x: self.x % other.x,
            y: self.y % other.y,
            z: self.z % other.z,
        }
    }
}

impl<T> ops::Rem<T> for Vector3<T> where
    T: Num + Copy
{
    type Output = Self;

    fn rem(self, value: T) -> Self::Output {
        Self {
            x: self.x % value,
            y: self.y % value,
            z: self.z % value,
        }
    }
}

impl<T> ops::AddAssign for Vector3<T> where
    T: Num + Copy
{
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T> ops::AddAssign<T> for Vector3<T> where
    T: Num + Copy
{
    fn add_assign(&mut self, value: T) {
        *self = Self {
            x: self.x + value,
            y: self.y + value,
            z: self.z + value,
        }
    }
}

impl<T> ops::SubAssign for Vector3<T> where
    T: Num + Copy
{
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T> ops::SubAssign<T> for Vector3<T> where
    T: Num + Copy
{
    fn sub_assign(&mut self, value: T) {
        *self = Self {
            x: self.x - value,
            y: self.y - value,
            z: self.z - value,
        }
    }
}

impl<T> ops::MulAssign for Vector3<T> where
    T: Num + Copy
{
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl<T> ops::MulAssign<T> for Vector3<T> where
    T: Num + Copy
{
    fn mul_assign(&mut self, value: T) {
        *self = Self {
            x: self.x * value,
            y: self.y * value,
            z: self.z * value,
        }
    }
}

impl<T> ops::DivAssign for Vector3<T> where
    T: Num + Copy
{
    fn div_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl<T> ops::DivAssign<T> for Vector3<T> where
    T: Num + Copy
{
    fn div_assign(&mut self, value: T) {
        *self = Self {
            x: self.x / value,
            y: self.y / value,
            z: self.z / value,
        }
    }
}

impl<T> ops::RemAssign for Vector3<T> where
    T: Num + Copy
{
    fn rem_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x % other.x,
            y: self.y % other.y,
            z: self.z % other.z,
        }
    }
}

impl<T> ops::RemAssign<T> for Vector3<T> where
    T: Num + Copy
{
    fn rem_assign(&mut self, value: T) {
        *self = Self {
            x: self.x % value,
            y: self.y % value,
            z: self.z % value,
        }
    }
}

impl<T> ops::Index<usize> for Vector3<T> where
    T: Num
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index was out of range, it must be in range [0, 2]")
        }
    }
}

impl<T> ops::IndexMut<usize> for Vector3<T> where
    T: Num
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index was out of range, it must be in range [0, 2]")
        }
    }
}

impl<T> From<(T, T, T)> for Vector3<T> where
    T: Num + Copy
{
    fn from(tuple: (T, T, T)) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
            z: tuple.2,
        }
    }
}

impl<T> From<&(T, T, T)> for Vector3<T> where
    T: Num + Copy
{
    fn from(tuple: &(T, T, T)) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
            z: tuple.2,
        }
    }
}

impl<T> From<[T; 3]> for Vector3<T> where
    T: Num + Copy
{
    fn from(slice: [T; 3]) -> Self {
        Self {
            x: slice[0],
            y: slice[1],
            z: slice[2],
        }
    }
}

impl<T> From<&[T; 3]> for Vector3<T> where
    T: Num + Copy
{
    fn from(slice: &[T; 3]) -> Self {
        Self {
            x: slice[0],
            y: slice[1],
            z: slice[2],
        }
    }
}
