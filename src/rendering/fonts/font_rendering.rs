use std::collections::HashMap;
use super::{Glyph, Texture};

pub trait FontRendering {
    fn texture<'t>(&'t self) -> Option<&'t Texture>;
    fn glyphs(&self) -> HashMap<u32, Glyph>;
    fn ascender(&self) -> f32;
    fn descender(&self) -> f32;
    fn nominal_width(&self) -> f32;
    fn line_height(&self) -> f32;
    fn has_kerning(&self) -> bool;
    fn kerning(&self, unicode: u32, next_unicode: u32) -> f64;
}
