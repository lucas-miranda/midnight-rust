use std::marker::PhantomData;

use crate::{
    math::Vector2,
    rendering::{
        VertexPosition,
        VertexTexture2D,
    },
    util::Size,
};

use super::{
    Graphic,
    DrawConfig,
    RenderState,
    Texture, GraphicDrawError,
};

pub struct Image<V: VertexPosition<Position = Vector2<f32>>> {
    pub texture: Texture,
    pub phantom: PhantomData<V>,
}

/*
impl<V: VertexPosition<Position = Vector2<f32>>> Graphic<V> for Image<V> {
    fn texture<'t>(&'t self) -> Option<&'t Texture> {
        Some(&self.texture)
    }

    fn draw<'d>(
        &'d self,
        state: &'d mut dyn RenderState<V>,
        draw_config: DrawConfig<V>,
    ) {
        let size: Size<f32>
            = Size::with(self.texture.width(), self.texture.height()).unwrap();

        println!("image size: {}", size);
        let vertices = vec![
            V::from_position(Vector2::new(0.0, 0.0)),
            V::from_position(Vector2::new(size.width, 0.0)),
            V::from_position(Vector2::new(0.0, size.height)),

            V::from_position(Vector2::new(0.0, size.height)),
            V::from_position(Vector2::new(size.width, 0.0)),
            V::from_position(Vector2::new(size.width, size.height)),
        ];

        state.extend(
            vertices.iter(),
            None,
            //Some(&self.texture),
            draw_config
        );
    }
}
*/

impl<V: VertexPosition<Position = Vector2<f32>> + VertexTexture2D> Graphic<V> for Image<V> {
    fn texture<'t>(&'t self) -> Option<&'t Texture> {
        Some(&self.texture)
    }

    fn draw<'d>(
        &'d self,
        state: &'d mut dyn RenderState<V>,
        draw_config: DrawConfig<V>,
    ) -> Result<(), GraphicDrawError> {
        let size: Size<f32>
            = Size::with(self.texture.width(), self.texture.height()).unwrap();

        println!("image size: {}", size);
        let vertices = vec![
            V::from_position(Vector2::new(0.0, 0.0)).with_uv(Vector2::new(0.0, 0.0)),
            V::from_position(Vector2::new(size.width, 0.0)).with_uv(Vector2::new(1.0, 0.0)),
            V::from_position(Vector2::new(0.0, size.height)).with_uv(Vector2::new(0.0, 1.0)),

            V::from_position(Vector2::new(0.0, size.height)).with_uv(Vector2::new(0.0, 1.0)),
            V::from_position(Vector2::new(size.width, 0.0)).with_uv(Vector2::new(1.0, 0.0)),
            V::from_position(Vector2::new(size.width, size.height)).with_uv(Vector2::new(1.0, 1.0)),
        ];

        state.extend(vertices.iter(), Some(&self.texture), draw_config)
             .map_err(GraphicDrawError::from)?;

        Ok(())
    }
}
