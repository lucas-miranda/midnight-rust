use crate::{
    math::Vector2,
    util::Size,
};

use super::{
    Graphic,
    DrawConfig,
    RenderState,
};

pub struct Grid {
    pub columns: u32,
    pub rows: u32,
    pub tile_size: Size<u32>,
}

impl Grid {
}

impl Graphic for Grid {
    fn draw<'d>(
        &'d self,
        state: &'d mut dyn RenderState,
        mut draw_config: DrawConfig,
    ) {
        let origin = Vector2::new(0.0, 0.0);
        draw_config.shader_id = 1;

        state.extend(
            vec!(
                origin,
                origin + Vector2::new(0, self.rows * self.tile_size.height).convert(),

                origin + Vector2::new(self.columns * self.tile_size.width, self.rows * self.tile_size.height).convert(),
                origin,

                origin + Vector2::new(self.columns * self.tile_size.width, self.rows * self.tile_size.height).convert(),
                origin + Vector2::new(self.columns * self.tile_size.width, 0).convert(),
            ).iter(),
            draw_config,
        )
    }
}
