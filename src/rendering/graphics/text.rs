use std::{marker::PhantomData, any::Any};

use crate::{
    math::{Vector2, Rectangle},
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
        let text_bytes = self.text.as_bytes();

        if text_bytes.len() == 0 {
            return Ok(());
        }

        let texture_size: Vector2<f32>;

        match self.texture() {
            Some(texture) => texture_size = Vector2::with(texture.size().width, texture.size().height).unwrap(),
            None => return Ok(()),
        }

        if let Some(glyph) = self.font.glyph(text_bytes[0] as u32) {
            let font_size_ratio = self.font.size / self.font.rendering.nominal_width();
            let glyph_size: Vector2<f32> = glyph.source_area.size.convert();
            let quad_size = glyph_size * font_size_ratio;

            //println!("glyph_size: {}, font_size_ratio: {}, texture_size: {}", glyph_size, font_size_ratio, texture_size);

            let uv = Rectangle::new(
                glyph.source_area.position.convert::<f32>() / texture_size,
                glyph_size / texture_size
            );

            //let em_size = Vector2::new(glyph.advance_x, glyph.advance_y + self.font.rendering.descender() as f64);
            //let unscaled_px_size = Vector2::new(em_size.x * self.font.size as f64, em_size.y * self.font.size as f64);

            let vertices = vec![
            /*
                0---1
                |  /
                | /
                |/
                2
             */

                V::from_position(Vector2::new(0.0, 0.0)).with_uv(uv.top_left()),
                V::from_position(Vector2::new(quad_size.x, 0.0)).with_uv(uv.top_right()),
                V::from_position(Vector2::new(0.0, quad_size.y)).with_uv(uv.bottom_left()),

            /*
                    4
                   /|
                  / |
                 /  |
                3---5
             */

                V::from_position(Vector2::new(0.0, quad_size.y)).with_uv(uv.bottom_left()),
                V::from_position(Vector2::new(quad_size.x, 0.0)).with_uv(uv.top_right()),
                V::from_position(Vector2::new(quad_size.x, quad_size.y)).with_uv(uv.bottom_right()),
            ];

            state.extend(vertices.iter(), self.texture(), draw_config)
                 .map_err(GraphicDrawError::from)?;
        }

        /*
        let size: Size<f32>
            = Size::with(self.texture.width(), self.texture.height()).unwrap();

        //println!("image size: {}", size);
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
        */

        Ok(())
    }
}
