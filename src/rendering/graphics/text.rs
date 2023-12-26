use std::{marker::PhantomData, any::Any};

use crate::{
    math::{Vector2, Rectangle, Size2},
    rendering::{
        fonts::{Font, FontRendering},
        VertexPosition,
        VertexTexture2D,
    },
};

use super::{
    DrawConfig,
    Graphic,
    GraphicDrawError,
    RenderState,
    Texture,
};

pub struct Text<R, V> where
    R: FontRendering + 'static,
    V: VertexPosition<Position = Vector2<f32>> + VertexTexture2D,
{
    pub font: Font<R>,
    pub text: String,
    pub phantom: PhantomData<V>,
}

impl<R, V> Text<R, V> where
    R: FontRendering + 'static,
    V: VertexPosition<Position = Vector2<f32>> + VertexTexture2D,
{
}

impl<R, V> Graphic<V> for Text<R, V> where
    R: FontRendering,
    V: VertexPosition<Position = Vector2<f32>> + VertexTexture2D,
{
    fn texture<'t>(&'t self) -> Option<&'t Texture> {
        self.font.rendering.texture()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn draw<'d>(
        &'d self,
        state: &'d mut dyn RenderState<V>,
        draw_config: DrawConfig<V>,
    ) -> Result<(), GraphicDrawError> {
        if self.text.is_empty() {
            return Ok(());
        }

        let texture_size: Vector2<f32>;

        match self.texture() {
            Some(texture) => texture_size = Vector2::with(texture.size().width, texture.size().height).unwrap(),
            None => return Ok(()),
        }

        let (render_data, _render_em_size) = self.font.build_text(&self.text);
        let mut vertices = Vec::with_capacity(render_data.len() * 3 * 2); // 3 vertices by 2 tri

        let font_size_ratio = self.font.size / self.font.rendering.nominal_width();

        for render_glyph in &render_data {
            let glyph_size: Vector2<f32> = render_glyph.source_area.size.convert();
            let quad_size = glyph_size * font_size_ratio;

            let uv = Rectangle::new(
                render_glyph.source_area.position.convert::<f32>() / texture_size,
                glyph_size / texture_size
            );

            let pos = (render_glyph.position * self.font.size as f64).convert();

            vertices.extend_from_slice(&[
                /*
                   0---1
                   |  /
                   | /
                   |/
                   2
                */

                V::from_position(pos).with_uv(uv.top_left()),
                V::from_position(pos + Vector2::new(quad_size.x, 0.0)).with_uv(uv.top_right()),
                V::from_position(pos + Vector2::new(0.0, quad_size.y)).with_uv(uv.bottom_left()),

                /*
                       4
                      /|
                     / |
                    /  |
                   3---5
                */

                V::from_position(pos + Vector2::new(0.0, quad_size.y)).with_uv(uv.bottom_left()),
                V::from_position(pos + Vector2::new(quad_size.x, 0.0)).with_uv(uv.top_right()),
                V::from_position(pos + Vector2::new(quad_size.x, quad_size.y)).with_uv(uv.bottom_right()),
            ]);
        }

        state.extend(vertices.iter(), self.texture(), draw_config)
             .map_err(GraphicDrawError::from)?;

        Ok(())
    }
}
