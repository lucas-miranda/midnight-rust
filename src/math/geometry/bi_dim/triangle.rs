use std::{ops, fmt::{self, Display}};

use crate::math::num_traits::{
    cast::{cast, NumCast},
    Num,
};

use super::Point;

/// A 3 vertices polygon.
/// Shorthand to Triangle<T>.
pub type Tri<T> = Triangle<T>;

/// A 3 vertices polygon.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct Triangle<T> where
    T: Num
{
    pub a: Point<T>,
    pub b: Point<T>,
    pub c: Point<T>,
}

impl<T> Triangle<T> where
    T: Num
{
    pub const fn new(a: Point<T>, b: Point<T>, c: Point<T>) -> Self {
        Self {
            a,
            b,
            c,
        }
    }
}

impl<T> Triangle<T> where
    T: Num + NumCast
{
    pub fn with<U>(a: Point<U>, b: Point<U>, c: Point<U>) -> Option<Self> where
        U: Num + NumCast
    {
        Some(Self::new(
            Point { x: cast::<U, T>(a.x)?, y: cast::<U, T>(a.y)? },
            Point { x: cast::<U, T>(b.x)?, y: cast::<U, T>(b.y)? },
            Point { x: cast::<U, T>(c.x)?, y: cast::<U, T>(c.y)? },
        ))
    }
}

impl<T: Num + Display> Display for Triangle<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}, {}, {}", self.a, self.b, self.c)
    }
}

impl<T> ops::Add<T> for Triangle<T> where
    T: Num + Copy
{
    type Output = Self;

    fn add(self, value: T) -> Self::Output {
        Self {
            a: self.a + value,
            b: self.b + value,
            c: self.c + value,
        }
    }
}

impl<T> ops::Sub<T> for Triangle<T> where
    T: Num + Copy
{
    type Output = Self;

    fn sub(self, value: T) -> Self::Output {
        Self {
            a: self.a - value,
            b: self.b - value,
            c: self.c - value,
        }
    }
}

impl<T> ops::Mul<T> for Triangle<T> where
    T: Num + Copy
{
    type Output = Self;

    fn mul(self, value: T) -> Self::Output {
        Self {
            a: self.a * value,
            b: self.b * value,
            c: self.c * value,
        }
    }
}

impl<T> ops::Div<T> for Triangle<T> where
    T: Num + Copy
{
    type Output = Self;

    fn div(self, value: T) -> Self::Output {
        Self {
            a: self.a / value,
            b: self.b / value,
            c: self.c / value,
        }
    }
}

impl<T> ops::Rem<T> for Triangle<T> where
    T: Num + Copy
{
    type Output = Self;

    fn rem(self, value: T) -> Self::Output {
        Self {
            a: self.a % value,
            b: self.b % value,
            c: self.c % value,
        }
    }
}

impl<T> ops::AddAssign<T> for Triangle<T> where
    T: Num + Copy
{
    fn add_assign(&mut self, value: T) {
        *self = Self {
            a: self.a + value,
            b: self.b + value,
            c: self.c + value,
        }
    }
}

impl<T> ops::SubAssign<T> for Triangle<T> where
    T: Num + Copy
{
    fn sub_assign(&mut self, value: T) {
        *self = Self {
            a: self.a - value,
            b: self.b - value,
            c: self.c - value,
        }
    }
}

impl<T> ops::MulAssign<T> for Triangle<T> where
    T: Num + Copy
{
    fn mul_assign(&mut self, value: T) {
        *self = Self {
            a: self.a * value,
            b: self.b * value,
            c: self.c * value,
        }
    }
}

impl<T> ops::DivAssign<T> for Triangle<T> where
    T: Num + Copy
{
    fn div_assign(&mut self, value: T) {
        *self = Self {
            a: self.a / value,
            b: self.b / value,
            c: self.c / value,
        }
    }
}

impl<T> ops::RemAssign<T> for Triangle<T> where
    T: Num + Copy
{
    fn rem_assign(&mut self, value: T) {
        *self = Self {
            a: self.a % value,
            b: self.b % value,
            c: self.c % value,
        }
    }
}

impl<T> ops::Index<usize> for Triangle<T> where
    T: Num
{
    type Output = Point<T>;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.a,
            1 => &self.b,
            2 => &self.c,
            _ => panic!("Index was out of range, it must be in range [0, 2]")
        }
    }
}

impl<T> ops::IndexMut<usize> for Triangle<T> where
    T: Num
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.a,
            1 => &mut self.b,
            2 => &mut self.c,
            _ => panic!("Index was out of range, it must be in range [0, 2]")
        }
    }
}

impl<T> From<(Point<T>, Point<T>, Point<T>)> for Triangle<T> where
    T: Num + Copy
{
    fn from(tuple: (Point<T>, Point<T>, Point<T>)) -> Self {
        Self {
            a: tuple.0,
            b: tuple.1,
            c: tuple.2,
        }
    }
}

impl<T> From<&(Point<T>, Point<T>, Point<T>)> for Triangle<T> where
    T: Num + Copy
{
    fn from(tuple: &(Point<T>, Point<T>, Point<T>)) -> Self {
        Self {
            a: tuple.0,
            b: tuple.1,
            c: tuple.2,
        }
    }
}

impl<T> From<[Point<T>; 3]> for Triangle<T> where
    T: Num + Copy
{
    fn from(slice: [Point<T>; 3]) -> Self {
        Self {
            a: slice[0],
            b: slice[1],
            c: slice[2],
        }
    }
}

impl<T> From<&[Point<T>; 3]> for Triangle<T> where
    T: Num + Copy
{
    fn from(slice: &[Point<T>; 3]) -> Self {
        Self {
            a: slice[0],
            b: slice[1],
            c: slice[2],
        }
    }
}
