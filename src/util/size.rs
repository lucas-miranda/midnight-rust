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
