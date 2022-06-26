use std::cell::RefMut;
use num_traits::Num;

use crate::{
    rendering::{shaders::Shader, GraphicAdapter},
    math::Triangle,
};


pub trait Graphic {
    fn draw(&self, graphic_adapter: RefMut<GraphicAdapter>, shader: &Shader);
}

//

/*
// TODO  maybe it should share math::Triangle struct?
pub struct Triangle<T: Num + Copy> {
    pub a: Vec2<T>,
    pub b: Vec2<T>,
    pub c: Vec2<T>,
}

impl<T: Num + Copy> Triangle<T> {
    pub fn new(a: Vec2<T>, b: Vec2<T>, c: Vec2<T>) -> Self {
        Self {
            a,
            b,
            c,
        }
    }
}
*/

impl<T: Num + Copy> Graphic for Triangle<T> {
    fn draw(&self, mut graphic_adapter: RefMut<GraphicAdapter>, shader: &Shader) {
        graphic_adapter.draw_vertices(&[self.a, self.b, self.c], shader);
    }
}
