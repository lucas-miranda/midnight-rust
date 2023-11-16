use std::slice::Iter;
use crate::math::{Vector2, Rectangle, Size};

use super::Glyph;


pub struct TextRenderData {
    glyphs: Vec<RenderGlyph>,
}

impl TextRenderData {
    pub fn new(length: usize) -> Self {
        Self {
            glyphs: Vec::with_capacity(length),
        }
    }

    pub fn len(&self) -> usize {
        self.glyphs.len()
    }

    pub fn append(
        &mut self,
        position: Vector2<f64>,
        unicode: u32,
        data: &Glyph,
    ) {
        self.glyphs.push(RenderGlyph::new(position, unicode, data));
    }
}

impl<'a> IntoIterator for &'a TextRenderData {
    type Item = &'a RenderGlyph;
    type IntoIter = Iter<'a, RenderGlyph>;

    fn into_iter(self) -> Self::IntoIter {
        self.glyphs.iter()
    }
}


pub struct RenderGlyph {
    pub position: Vector2<f64>,
    pub original_position: Vector2<f64>,
    pub unicode: u32,

    // from fonts::Glyph
    pub source_area: Rectangle<f64>,
    pub bearing: Vector2<f64>,
    pub size: Size<f64>,
    pub advance: Vector2<f64>,
}

impl RenderGlyph {
    fn new(position: Vector2<f64>, unicode: u32, data: &Glyph) -> Self {
        Self {
            position,
            original_position: position,
            unicode,

            source_area: data.source_area,
            bearing: data.bearing,
            size: data.size,
            advance: data.advance,
        }
    }
}
