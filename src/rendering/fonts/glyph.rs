use crate::math::Rectangle;

#[derive(Debug)]
pub struct Glyph {
    pub source_area: Rectangle<f64>,
    pub bearing_x: f64,
    pub bearing_y: f64,
    pub width: f64,
    pub height: f64,
    pub advance_x: f64,
    pub advance_y: f64,
}
