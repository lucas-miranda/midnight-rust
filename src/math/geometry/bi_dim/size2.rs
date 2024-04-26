use std::{
    ops,
    fmt::{
        self,
        Display,
    },
};

use bytemuck::{Pod, Zeroable};
use winit::dpi::LogicalSize;

use crate::math::num_traits::{
    cast::{
        cast,
        NumCast,
    },
    Num,
};

use super::Vector2;

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct Size2<T> where
    T: Num
{
    pub width: T,
    pub height: T,
}

impl<T> Size2<T> where
    T: Num
{
    pub const fn new(width: T, height: T) -> Self {
        Self {
            width,
            height,
        }
    }
}

impl<T> Size2<T> where
    T: Num + NumCast
{
    pub fn with<U>(width: U, height: U) -> Option<Self> where
        U: Num + NumCast
    {
        Some(Self::new(
            cast::<U, T>(width)?,
            cast::<U, T>(height)?,
        ))
    }

    pub fn convert<U>(self) -> Size2<U> where
        U: Num + NumCast
    {
        self.try_convert().unwrap()
    }

    pub fn try_convert<U>(self) -> Option<Size2<U>> where
        U: Num + NumCast
    {
        Some(Size2::<U>::new(
            cast::<T, U>(self.width)?,
            cast::<T, U>(self.height)?,
        ))
    }
}

impl<T: Num + Display> Display for Size2<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}, {}", self.width, self.height)
    }
}

impl<T> ops::Neg for Size2<T> where
    T: Num + ops::Neg<Output = T>
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            width: -self.width,
            height: -self.height,
        }
    }
}

impl<T> ops::Add for Size2<T> where
    T: Num
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            width: self.width + other.width,
            height: self.height + other.height,
        }
    }
}

impl<T> ops::Add<T> for Size2<T> where
    T: Num + Copy
{
    type Output = Self;

    fn add(self, value: T) -> Self::Output {
        Self {
            width: self.width + value,
            height: self.height + value,
        }
    }
}

impl<T> ops::Sub for Size2<T> where
    T: Num
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            width: self.width - other.width,
            height: self.height - other.height,
        }
    }
}

impl<T> ops::Sub<T> for Size2<T> where
    T: Num + Copy
{
    type Output = Self;

    fn sub(self, value: T) -> Self::Output {
        Self {
            width: self.width - value,
            height: self.height - value,
        }
    }
}

impl<T> ops::Mul for Size2<T> where
    T: Num
{
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            width: self.width * other.width,
            height: self.height * other.height,
        }
    }
}

impl<T> ops::Mul<T> for Size2<T> where
    T: Num + Copy
{
    type Output = Self;

    fn mul(self, value: T) -> Self::Output {
        Self {
            width: self.width * value,
            height: self.height * value,
        }
    }
}

impl<T> ops::Div for Size2<T> where
    T: Num
{
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self {
            width: self.width / other.width,
            height: self.height / other.height,
        }
    }
}

impl<T> ops::Div<T> for Size2<T> where
    T: Num + Copy
{
    type Output = Self;

    fn div(self, value: T) -> Self::Output {
        Self {
            width: self.width / value,
            height: self.height / value,
        }
    }
}

impl<T> ops::Rem for Size2<T> where
    T: Num
{
    type Output = Self;

    fn rem(self, other: Self) -> Self::Output {
        Self {
            width: self.width % other.width,
            height: self.height % other.height,
        }
    }
}

impl<T> ops::Rem<T> for Size2<T> where
    T: Num + Copy
{
    type Output = Self;

    fn rem(self, value: T) -> Self::Output {
        Self {
            width: self.width % value,
            height: self.height % value,
        }
    }
}

impl<T> ops::AddAssign for Size2<T> where
    T: Num + Copy
{
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            width: self.width + other.width,
            height: self.height + other.height,
        }
    }
}

impl<T> ops::AddAssign<T> for Size2<T> where
    T: Num + Copy
{
    fn add_assign(&mut self, value: T) {
        *self = Self {
            width: self.width + value,
            height: self.height + value,
        }
    }
}

impl<T> ops::SubAssign for Size2<T> where
    T: Num + Copy
{
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            width: self.width - other.width,
            height: self.height - other.height,
        }
    }
}

impl<T> ops::SubAssign<T> for Size2<T> where
    T: Num + Copy
{
    fn sub_assign(&mut self, value: T) {
        *self = Self {
            width: self.width - value,
            height: self.height - value,
        }
    }
}

impl<T> ops::MulAssign for Size2<T> where
    T: Num + Copy
{
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            width: self.width * other.width,
            height: self.height * other.height,
        }
    }
}

impl<T> ops::MulAssign<T> for Size2<T> where
    T: Num + Copy
{
    fn mul_assign(&mut self, value: T) {
        *self = Self {
            width: self.width * value,
            height: self.height * value,
        }
    }
}

impl<T> ops::DivAssign for Size2<T> where
    T: Num + Copy
{
    fn div_assign(&mut self, other: Self) {
        *self = Self {
            width: self.width / other.width,
            height: self.height / other.height,
        }
    }
}

impl<T> ops::DivAssign<T> for Size2<T> where
    T: Num + Copy
{
    fn div_assign(&mut self, value: T) {
        *self = Self {
            width: self.width / value,
            height: self.height / value,
        }
    }
}

impl<T> ops::RemAssign for Size2<T> where
    T: Num + Copy
{
    fn rem_assign(&mut self, other: Self) {
        *self = Self {
            width: self.width % other.width,
            height: self.height % other.height,
        }
    }
}

impl<T> ops::RemAssign<T> for Size2<T> where
    T: Num + Copy
{
    fn rem_assign(&mut self, value: T) {
        *self = Self {
            width: self.width % value,
            height: self.height % value,
        }
    }
}

impl<T> ops::Index<usize> for Size2<T> where
    T: Num
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.width,
            1 => &self.height,
            _ => panic!("Index was out of range, it must be in range [0, 1]")
        }
    }
}

impl<T> ops::IndexMut<usize> for Size2<T> where
    T: Num
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.width,
            1 => &mut self.height,
            _ => panic!("Index was out of range, it must be in range [0, 1]")
        }
    }
}

impl<T> From<(T, T)> for Size2<T> where
    T: Num + Copy
{
    fn from(tuple: (T, T)) -> Self {
        Self {
            width: tuple.0,
            height: tuple.1,
        }
    }
}

impl<T> From<&(T, T)> for Size2<T> where
    T: Num + Copy
{
    fn from(tuple: &(T, T)) -> Self {
        Self {
            width: tuple.0,
            height: tuple.1,
        }
    }
}

impl<T> From<[T; 2]> for Size2<T> where
    T: Num + Copy
{
    fn from(slice: [T; 2]) -> Self {
        Self {
            width: slice[0],
            height: slice[1],
        }
    }
}

impl<T> From<&[T; 2]> for Size2<T> where
    T: Num + Copy
{
    fn from(slice: &[T; 2]) -> Self {
        Self {
            width: slice[0],
            height: slice[1],
        }
    }
}

impl<T> From<Vector2<T>> for Size2<T> where
    T: Num + Copy
{
    fn from(v: Vector2<T>) -> Self {
        Self {
            width: v.x,
            height: v.y,
        }
    }
}

impl<T> From<LogicalSize<T>> for Size2<T> where
    T: Num + Copy
{
    fn from(logical: LogicalSize<T>) -> Self {
        Self {
            width: logical.width,
            height: logical.height,
        }
    }
}

unsafe impl Zeroable for Size2<f32> {
}

unsafe impl Pod for Size2<f32> {
}
