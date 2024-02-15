use std::{marker::PhantomData, any::Any};

use crate::{
    math::Vector2,
    rendering::VertexPosition,
    util::Size,
};

use super::{
    DrawConfig,
    Graphic,
    GraphicDrawError,
    RenderState,
    Texture,
};

pub struct Grid<V: VertexPosition<Position = Vector2<f32>>> {
    pub columns: u32,
    pub rows: u32,
    pub tile_size: Size<u32>,
    pub phantom: PhantomData<V>,
}

impl<V: VertexPosition<Position = Vector2<f32>>> Graphic<V> for Grid<V> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn draw<'d>(
        &'d self, state: &'d mut dyn RenderState<V>,
        draw_config: DrawConfig<V>,
    ) -> Result<(), GraphicDrawError>{
        let mut vertices = Vec::with_capacity(((self.columns * 2) * (self.rows * 2)) as usize);

        // rows lines
        (0..=self.rows).for_each(|r| {
            let y = (r * self.tile_size.height) as f32;

            vertices.extend(&[
                V::from_position(Vector2::new(0.0, y)),
                V::from_position(Vector2::new((self.columns * self.tile_size.width) as f32, y)),
            ])
        });

        // columns lines
        (0..=self.columns).for_each(|c| {
            let x = (c * self.tile_size.width) as f32;

            vertices.extend(&[
                V::from_position(Vector2::new(x, 0.0)),
                V::from_position(Vector2::new(x, (self.rows * self.tile_size.height) as f32)),
            ])
        });

        state.extend(vertices.iter(), None, draw_config)
             .map_err(GraphicDrawError::from)?;

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

        Ok(())
    }
}
