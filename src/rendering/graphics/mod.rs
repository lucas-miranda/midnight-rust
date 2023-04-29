mod grid;
pub use grid::Grid;

use crate::math::Triangle;
use super::backend::RenderPass;

pub trait Graphic {
    fn draw<'d>(
        &'d self,
        pass: RenderPass<'d>,
    ) -> RenderPass<'d>;
}

//

impl Graphic for Triangle<f32> {
    fn draw<'d>(
        &'d self,
        pass: RenderPass<'d>,
    ) -> RenderPass<'d> {
        pass.extend_vertices(vec!(self.a, self.b, self.c))
    }
}
