use std::{collections::HashMap, rc::Weak};
use crate::math::Size2;

use super::{Glyph, Texture};

pub trait FontRendering {
    fn texture(&self) -> Option<Weak<Texture>>;
    fn texture_size(&self) -> Option<Size2<u32>>;
    fn glyphs(&self) -> HashMap<u32, Glyph>;
    fn ascender(&self) -> f32;
    fn descender(&self) -> f32;
    fn nominal_width(&self) -> f32;
    fn line_height(&self) -> f32;
    fn has_kerning(&self) -> bool;
    fn kerning(&self, unicode: u32, next_unicode: u32) -> f64;
}
