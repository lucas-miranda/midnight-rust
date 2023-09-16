use std::fmt::{
    self,
    Display,
};

use crate::math::{num_traits::{
    Num,
    cast::{
        cast,
        NumCast,
    },
}, Vector2};

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct Size<T> where
    T: Num
{
    pub width: T,
    pub height: T,
}

impl<T> Size<T> where
    T: Num
{
    pub const fn new(width: T, height: T) -> Self {
        Self {
            width,
            height,
        }
    }
}

impl<T> Size<T> where
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

impl<T: Num + Display> Display for Size<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}, {}", self.width, self.height)
    }
}

impl<T> From<&(T, T)> for Size<T> where
    T: Num + Copy
{
    fn from(tuple: &(T, T)) -> Self {
        Self {
            width: tuple.0,
            height: tuple.1,
        }
    }
}

impl<T> From<Size<T>> for Vector2<T> where
    T: Num
{
    fn from(size: Size<T>) -> Self {
        Self {
            x: size.width,
            y: size.height,
        }
    }
}
