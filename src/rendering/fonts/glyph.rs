use crate::math::{Rectangle, Vector2, Size};

#[derive(Debug)]
pub struct Glyph {
    pub source_area: Rectangle<f64>,
    pub bearing: Vector2<f64>,
    pub size: Size<f64>,
    pub advance: Vector2<f64>,
}
