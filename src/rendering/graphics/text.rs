use std::{marker::PhantomData, any::Any};

use crate::{
    math::{Vector2, Rectangle, Size2},
    rendering::{
        fonts::{Font, FontRendering, TextRenderData},
        VertexPosition,
        VertexTexture2D,
    },
};

use super::{
    DrawConfig,
    Graphic,
    GraphicDrawError,
    RenderState,
};

pub struct Text<R, V> where
    R: FontRendering + 'static,
    V: VertexPosition<Position = Vector2<f32>> + VertexTexture2D,
{
    font: Font<R>,
    phantom: PhantomData<V>,
    text: String,
    render_data: Option<TextRenderData>,
    em_size: Size2<f64>,
}

impl<R, V> Text<R, V> where
    R: FontRendering + 'static,
    V: VertexPosition<Position = Vector2<f32>> + VertexTexture2D,
{
    pub fn new(font: Font<R>) -> Self {
        Self {
            font,
            phantom: Default::default(),
            text: Default::default(),
            render_data: None,
            em_size: Default::default(),
        }
    }

    pub fn with_value(font: Font<R>, text: String) -> Self {
        let (render_data, em_size) = font.build_text(&text);

        Self {
            font,
            phantom: Default::default(),
            text,
            render_data: Some(render_data),
            em_size,
        }
    }

    pub fn value(&self) -> &str {
        self.text.as_str()
    }

    pub fn em_size(&self) -> Size2<f64> {
        self.em_size
    }

    pub fn px_size(&self) -> Size2<f32> {
        (self.em_size() * self.font.size() as f64).convert()
    }

    pub fn change_value(&mut self, new_text: String) {
        self.text = new_text;
        let (render_data, em_size) = self.font.build_text(&self.text);
        self.render_data = Some(render_data);
        self.em_size = em_size;
    }
}

impl<R, V> Graphic<V> for Text<R, V> where
    R: FontRendering,
    V: VertexPosition<Position = Vector2<f32>> + VertexTexture2D,
{
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

        match self.font.rendering.texture_size() {
            Some(tex_size) => texture_size = tex_size.convert().into(),
            None => return Ok(()),
        }

        if let Some(ref render_data) = self.render_data {
            let mut vertices = Vec::with_capacity(render_data.len() * 3 * 2); // 3 vertices by 2 tri

            let font_size_ratio = self.font.size() / self.font.rendering.nominal_width();

            for render_glyph in render_data {
                let glyph_size: Vector2<f32> = render_glyph.source_area.size.convert().into();
                let quad_size = glyph_size * font_size_ratio;

                let uv = Rectangle::new(
                    render_glyph.source_area.position.convert::<f32>() / texture_size,
                    (glyph_size / texture_size).into(),
                );

                let pos = (render_glyph.position * self.font.size() as f64).convert();

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

            state.extend(vertices.iter(), self.font.rendering.texture(), draw_config)
                 .map_err(GraphicDrawError::from)?;
        }

        Ok(())
    }
}
