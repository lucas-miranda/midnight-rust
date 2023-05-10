use wgpu::PrimitiveTopology;

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
        draw_config: DrawConfig,
    ) {
        //let origin = Vector2::new(0.0, 0.0);
        //let mut shader_config = draw_config.shader_config.unwrap();
        //shader_config.primitive_state().topology = PrimitiveTopology::LineList;

        let mut vertices = Vec::with_capacity(((self.columns * 2) * (self.rows * 2)) as usize);

        // rows lines
        (0..=self.rows).for_each(|r| {
            let y = (r * self.tile_size.height) as f32;

            vertices.extend(&[
                Vector2::new(0.0, y),
                Vector2::new((self.columns * self.tile_size.width) as f32, y),
            ])
        });

        // columns lines
        (0..=self.columns).for_each(|c| {
            let x = (c * self.tile_size.width) as f32;

            vertices.extend(&[
                Vector2::new(x, 0.0),
                Vector2::new(x, (self.rows * self.tile_size.height) as f32),
            ])
        });

        state.extend(vertices.iter(), draw_config)

        /*
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
        */
    }
}
