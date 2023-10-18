use std::{marker::PhantomData, any::Any};

use crate::{
    math::Vector2,
    rendering::{VertexPosition, VertexTexture2D},
    util::Size,
};

use super::{
    DrawConfig,
    Graphic,
    GraphicDrawError,
    RenderState,
    Texture,
};

pub struct Tilemap<V: VertexPosition<Position = Vector2<f32>>> {
    pub columns: u32,
    pub rows: u32,
    pub tile_size: Size<u32>,
    pub tileset: Texture,
    pub tiles: Vec<u32>,
    pub phantom: PhantomData<V>,
}

impl<V: VertexPosition<Position = Vector2<f32>>> Tilemap<V> {
    pub fn new(columns: u32, rows: u32, tile_size: Size<u32>, tileset: Texture) -> Self {
        Self {
            columns,
            rows,
            tile_size,
            tileset,
            tiles: Vec::new(),
            phantom: Default::default(),
        }
    }

    pub fn set_tiles_coord(&mut self, tiles: Vec<Vector2<u32>>) {
        let tileset_columns = self.tileset.width() / self.tile_size.width;

        self.tiles = tiles.into_iter()
                          .map(|t| t.y * tileset_columns + t.x)
                          .collect()
    }
}

impl<V> Graphic<V> for Tilemap<V> where
    V: VertexPosition<Position = Vector2<f32>> + VertexTexture2D,
{
    fn texture<'t>(&'t self) -> Option<&'t Texture> {
        Some(&self.tileset)
    }

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
        let tile_size: Size<f32> = Size::with(self.tile_size.width, self.tile_size.height).unwrap();

        let tileset_columns = self.tileset.width() / self.tile_size.width;
        //let tileset_rows = self.tileset.height() / self.tile_size.height;

        let tileset_tile_uv_size = tile_size / Size::<f32>::with(self.tileset.width(), self.tileset.height()).unwrap();

        // rows lines
        (0..self.rows).for_each(|r| {
            let y = (r * self.tile_size.height) as f32;

            (0..self.columns).for_each(|c| {
                let x = (c * self.tile_size.width) as f32;

                let index: usize = (c + r * self.columns) as usize;
                let gid = self.tiles[index];

                let g_x = gid % tileset_columns;
                let g_y = gid / tileset_columns;

                let uv = Vector2::new(
                    (g_x as f32) * tile_size.width / (self.tileset.width() as f32),
                    (g_y as f32) * tile_size.height / (self.tileset.height() as f32)
                );

                vertices.extend(&[
                    V::from_position(Vector2::new(x, y)).with_uv(uv),
                    V::from_position(Vector2::new(x + tile_size.width, y)).with_uv(uv + Vector2::new(tileset_tile_uv_size.width, 0.0)),
                    V::from_position(Vector2::new(x, y + tile_size.height)).with_uv(uv + Vector2::new(0.0, tileset_tile_uv_size.height)),

                    V::from_position(Vector2::new(x, y + tile_size.height)).with_uv(uv + Vector2::new(0.0, tileset_tile_uv_size.height)),
                    V::from_position(Vector2::new(x + tile_size.width, y)).with_uv(uv + Vector2::new(tileset_tile_uv_size.width, 0.0)),
                    V::from_position(Vector2::new(x + tile_size.width, y + tile_size.height)).with_uv(uv + Vector2::<_>::from(tileset_tile_uv_size)),
                ])
            });
        });

        state.extend(vertices.iter(), Some(&self.tileset), draw_config)
             .map_err(GraphicDrawError::from)?;

        Ok(())
    }
}
