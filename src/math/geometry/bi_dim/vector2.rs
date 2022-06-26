use std::{ops, fmt::{self, Display}};

use crate::math::num_traits::{
    cast::{cast, NumCast},
    Num,
};

/// Describes a point on a bi-dimensional space.
/// Shorthand to Vector2<T>.
pub type Point<T> = Vector2<T>;

/// Describes a bi-dimensional value.
/// Shorthand to Vector2<T>.
pub type Vec2<T> = Vector2<T>;

/// Describes a bi-dimensional value.
///
/// It doesn't works as, or shares the same properties with, formal mathematics' vector.
///
/// It's interpretation depends on context, as it can be: a point in space, a direction,
/// a extent or others things.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct Vector2<T> where
    T: Num
{
    pub x: T,
    pub y: T,
}

impl<T> Vector2<T> where
    T: Num
{
    pub fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T> Vector2<T> where
    T: Num + NumCast
{
    pub fn with<U>(x: U, y: U) -> Option<Self> where
        U: Num + NumCast
    {
        Some(Self::new(
            cast::<U, T>(x)?,
            cast::<U, T>(y)?,
        ))
    }
}

impl<T: Num + Display> Display for Vector2<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}, {}", self.x, self.y)
    }
}

impl<T> ops::Neg for Vector2<T> where
    T: Num + ops::Neg<Output = T>
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<T> ops::Add for Vector2<T> where
    T: Num
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> ops::Add<T> for Vector2<T> where
    T: Num + Copy
{
    type Output = Self;

    fn add(self, value: T) -> Self::Output {
        Self {
            x: self.x + value,
            y: self.y + value,
        }
    }
}

impl<T> ops::Sub for Vector2<T> where
    T: Num
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T> ops::Sub<T> for Vector2<T> where
    T: Num + Copy
{
    type Output = Self;

    fn sub(self, value: T) -> Self::Output {
        Self {
            x: self.x - value,
            y: self.y - value,
        }
    }
}

impl<T> ops::Mul for Vector2<T> where
    T: Num
{
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl<T> ops::Mul<T> for Vector2<T> where
    T: Num + Copy
{
    type Output = Self;

    fn mul(self, value: T) -> Self::Output {
        Self {
            x: self.x * value,
            y: self.y * value,
        }
    }
}

impl<T> ops::Div for Vector2<T> where
    T: Num
{
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl<T> ops::Div<T> for Vector2<T> where
    T: Num + Copy
{
    type Output = Self;

    fn div(self, value: T) -> Self::Output {
        Self {
            x: self.x / value,
            y: self.y / value,
        }
    }
}

impl<T> ops::Rem for Vector2<T> where
    T: Num
{
    type Output = Self;

    fn rem(self, other: Self) -> Self::Output {
        Self {
            x: self.x % other.x,
            y: self.y % other.y,
        }
    }
}

impl<T> ops::Rem<T> for Vector2<T> where
    T: Num + Copy
{
    type Output = Self;

    fn rem(self, value: T) -> Self::Output {
        Self {
            x: self.x % value,
            y: self.y % value,
        }
    }
}

impl<T> ops::AddAssign for Vector2<T> where
    T: Num + Copy
{
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> ops::AddAssign<T> for Vector2<T> where
    T: Num + Copy
{
    fn add_assign(&mut self, value: T) {
        *self = Self {
            x: self.x + value,
            y: self.y + value,
        }
    }
}

impl<T> ops::SubAssign for Vector2<T> where
    T: Num + Copy
{
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T> ops::SubAssign<T> for Vector2<T> where
    T: Num + Copy
{
    fn sub_assign(&mut self, value: T) {
        *self = Self {
            x: self.x - value,
            y: self.y - value,
        }
    }
}

impl<T> ops::MulAssign for Vector2<T> where
    T: Num + Copy
{
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl<T> ops::MulAssign<T> for Vector2<T> where
    T: Num + Copy
{
    fn mul_assign(&mut self, value: T) {
        *self = Self {
            x: self.x * value,
            y: self.y * value,
        }
    }
}

impl<T> ops::DivAssign for Vector2<T> where
    T: Num + Copy
{
    fn div_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl<T> ops::DivAssign<T> for Vector2<T> where
    T: Num + Copy
{
    fn div_assign(&mut self, value: T) {
        *self = Self {
            x: self.x / value,
            y: self.y / value,
        }
    }
}

impl<T> ops::RemAssign for Vector2<T> where
    T: Num + Copy
{
    fn rem_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x % other.x,
            y: self.y % other.y,
        }
    }
}

impl<T> ops::RemAssign<T> for Vector2<T> where
    T: Num + Copy
{
    fn rem_assign(&mut self, value: T) {
        *self = Self {
            x: self.x % value,
            y: self.y % value,
        }
    }
}

impl<T> ops::Index<usize> for Vector2<T> where
    T: Num
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Index was out of range, it must be in range [0, 1]")
        }
    }
}

impl<T> ops::IndexMut<usize> for Vector2<T> where
    T: Num
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Index was out of range, it must be in range [0, 1]")
        }
    }
}

impl<T> From<(T, T)> for Vector2<T> where
    T: Num + Copy
{
    fn from(tuple: (T, T)) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

impl<T> From<&(T, T)> for Vector2<T> where
    T: Num + Copy
{
    fn from(tuple: &(T, T)) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

impl<T> From<[T; 2]> for Vector2<T> where
    T: Num + Copy
{
    fn from(slice: [T; 2]) -> Self {
        Self {
            x: slice[0],
            y: slice[1],
        }
    }
}

impl<T> From<&[T; 2]> for Vector2<T> where
    T: Num + Copy
{
    fn from(slice: &[T; 2]) -> Self {
        Self {
            x: slice[0],
            y: slice[1],
        }
    }
}
