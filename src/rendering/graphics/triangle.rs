use crate::math::Triangle;

use super::{
    Graphic,
    DrawConfig,
    RenderState,
};

impl Graphic for Triangle<f32> {
    fn draw<'d>(
        &'d self,
        state: &'d mut dyn RenderState,
        draw_config: DrawConfig,
    ) {
        state.extend(vec!(self.a, self.b, self.c).iter(), draw_config)
    }
}
