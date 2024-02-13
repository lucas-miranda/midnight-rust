mod geometry;
pub use geometry::*;

mod matrix4x4;
pub use matrix4x4::Matrix4x4;

pub use num_traits;

pub trait Approximable<T> {
    fn is_equal_approx(&self, to: &T) -> bool;
}

impl Approximable<f32> for f32 {
    fn is_equal_approx(&self, to: &f32) -> bool {
        (to - self).abs() <= f32::EPSILON
    }
}
