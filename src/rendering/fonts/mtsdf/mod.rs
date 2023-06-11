use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct MTSDF {
    atlas: Atlas,
    metrics: Metrics,
    glyphs: HashMap<u32, Glyph>
}

#[derive(Deserialize)]
pub struct Atlas {
    distance_range: f32,
    size: f32,
    width: f32,
    height: f32,
    y_origin: YOrigin,
}

#[derive(Deserialize)]
pub struct Metrics {
    em_size: f64,
    line_height: f32,
    ascender: f32,
    descender: f32,
    underline_y: f32,
    underline_thickness: f32,
}

#[derive(Deserialize)]
pub struct Glyph {
    unicode: u32,
    advance: f64,
    plane_bounds: GlyphBounds,
    atlas_bounds: GlyphBounds,
}

#[derive(Deserialize)]
pub struct GlyphBounds {
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub left: f64,
}

#[derive(Deserialize)]
pub enum YOrigin {
    Bottom,
    Top,
}
