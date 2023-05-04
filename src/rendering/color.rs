use std::fmt::{
    self,
    Display,
};

use bytemuck::{Pod, Zeroable};

use crate::math::{
    num_traits::Num,
    Vector4,
};

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct Color<T> where
    T: Num
{
    pub r: T,
    pub g: T,
    pub b: T,
    pub a: T,
}

impl<T> Color<T> where
    T: Num
{
    pub const fn rgba(r: T, g: T, b: T, a: T) -> Self {
        Self {
            r,
            g,
            b,
            a,
        }
    }
}

impl<T: Num + Display> Display for Color<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}, {}, {}, {}", self.r, self.g, self.b, self.a)
    }
}

impl<T> From<Vector4<T>> for Color<T> where
    T: Num
{
    fn from(v: Vector4<T>) -> Self {
        Self::rgba(v.x, v.y, v.z, v.w)
    }
}

impl<T> From<Color<T>> for Vector4<T> where
    T: Num
{
    fn from(c: Color<T>) -> Self {
        Self::new(c.r, c.g, c.b, c.a)
    }
}

// Color<u8>

#[allow(dead_code)]
impl Color<u8> {
    pub const WHITE: Self       = Self::rgba(255, 255, 255, 255);
    pub const BLACK: Self       = Self::rgba(0,     0,   0, 255);
    pub const RED: Self         = Self::rgba(255,   0,   0, 255);
    pub const GREEN: Self       = Self::rgba(0,   255,   0, 255);
    pub const BLUE: Self        = Self::rgba(0,     0, 255, 255);
    pub const CYAN: Self        = Self::rgba(0,   255, 255, 255);
    pub const MAGENTA: Self     = Self::rgba(255,   0, 255, 255);
    pub const YELLOW: Self      = Self::rgba(255, 255,   0, 255);

    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            r,
            g,
            b,
            a: 255,
        }
    }

    pub const fn rgba_hex(rgba: u32) -> Self {
        Self {
            r: ((rgba & 0xFF000000) >> 24) as u8,
            g: ((rgba & 0x00FF0000) >> 16) as u8,
            b: ((rgba & 0x0000FF00) >> 8) as u8,
            a: ((rgba & 0x000000FF)) as u8,
        }
    }

    pub const fn rgb_hex(rgb: u32) -> Self {
        Self {
            r: ((rgb & 0xFF0000) >> 16) as u8,
            g: ((rgb & 0x00FF00) >> 8) as u8,
            b: (rgb & 0x0000FF) as u8,
            a: 255,
        }
    }
}

impl From<Color<u8>> for wgpu::Color {
    fn from(c: Color<u8>) -> Self {
        Self {
            r: (c.r as f64) / 255.0,
            g: (c.g as f64) / 255.0,
            b: (c.b as f64) / 255.0,
            a: (c.a as f64) / 255.0,
        }
    }
}

// Bytemuck

/// Bytemuck implementation for regular types
macro_rules! impl_bytemuck {
    ($($type:ty),+) => {$(
        unsafe impl Zeroable for Color<$type> {
        }

        unsafe impl Pod for Color<$type> {
        }
    )+}
}

impl_bytemuck!(u8, f32, f64);

// Float impl

/// Implementation for float precision types
macro_rules! impl_float {
    ($($type:ty),+) => {$(
        #[allow(dead_code)]
        impl Color<$type> {
            pub const WHITE: Self       = Self::rgba(1.0, 1.0, 1.0, 1.0);
            pub const BLACK: Self       = Self::rgba(0.0, 0.0, 0.0, 1.0);
            pub const RED: Self         = Self::rgba(1.0, 0.0, 0.0, 1.0);
            pub const GREEN: Self       = Self::rgba(0.0, 1.0, 0.0, 1.0);
            pub const BLUE: Self        = Self::rgba(0.0, 0.0, 1.0, 1.0);
            pub const CYAN: Self        = Self::rgba(0.0, 1.0, 1.0, 1.0);
            pub const MAGENTA: Self     = Self::rgba(1.0, 0.0, 1.0, 1.0);
            pub const YELLOW: Self      = Self::rgba(1.0, 1.0, 0.0, 1.0);

            pub const fn rgb(r: $type, g: $type, b: $type) -> Self {
                Self {
                    r,
                    g,
                    b,
                    a: 1.0,
                }
            }

            pub const fn rgba_hex(rgba: u32) -> Self {
                Self {
                    r: ((rgba & 0xFF000000) >> 24) as $type,
                    g: ((rgba & 0x00FF0000) >> 16) as $type,
                    b: ((rgba & 0x0000FF00) >> 8) as $type,
                    a: (rgba & 0x000000FF) as $type,
                }
            }

            pub const fn rgb_hex(rgb: u32) -> Self {
                Self {
                    r: ((rgb & 0xFF0000) >> 16) as $type,
                    g: ((rgb & 0x00FF00) >> 8) as $type,
                    b: (rgb & 0x0000FF) as $type,
                    a: 1.0,
                }
            }
        }

        impl From<Color<u8>> for Color<$type> {
            fn from(c: Color<u8>) -> Self {
                Self {
                    r: (c.r as $type) / 255.0,
                    g: (c.g as $type) / 255.0,
                    b: (c.b as $type) / 255.0,
                    a: (c.a as $type) / 255.0,
                }
            }
        }

        impl From<Color<$type>> for Color<u8> {
            fn from(c: Color<$type>) -> Self {
                Self {
                    r: (c.r * 255.0) as u8,
                    g: (c.g * 255.0) as u8,
                    b: (c.b * 255.0) as u8,
                    a: (c.a * 255.0) as u8,
                }
            }
        }

        impl From<Color<$type>> for wgpu::Color {
            fn from(c: Color<$type>) -> Self {
                Self {
                    r: c.r.into(),
                    g: c.g.into(),
                    b: c.b.into(),
                    a: c.a.into(),
                }
            }
        }
    )+}

}

impl_float!(f32, f64);

impl From<Color<f64>> for Color<f32> {
    fn from(c: Color<f64>) -> Self {
        Self {
            r: c.r as f32,
            g: c.g as f32,
            b: c.b as f32,
            a: c.a as f32,
        }
    }
}

impl From<Color<f32>> for Color<f64> {
    fn from(c: Color<f32>) -> Self {
        Self {
            r: c.r.into(),
            g: c.g.into(),
            b: c.b.into(),
            a: c.a.into(),
        }
    }
}

// Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion() {
        let a = Color::rgba(70, 35, 110, 255);

        assert_eq!(
            Into::<Color<f32>>::into(a),
            Color::<f32>::rgba(
                (a.r as f32) / 255.0,
                (a.g as f32) / 255.0,
                (a.b as f32) / 255.0,
                (a.a as f32) / 255.0,
            )
        )
    }
}
