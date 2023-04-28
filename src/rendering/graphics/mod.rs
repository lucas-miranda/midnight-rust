mod grid;
pub use grid::Grid;

use std::cell::RefMut;

use crate::{
    rendering::{
        DrawConfig,
        GraphicAdapter,
    },
    math::Triangle,
};

use super::backend::{
    RenderPass,
    DrawCommand,
};


pub trait Graphic {
    fn draw<'d>(
        &'d self,
        //graphic_adapter: &'d mut RefMut<'_, GraphicAdapter>,
        draw_command: &'d mut DrawCommand,
        config: &'d DrawConfig,
    ) -> RenderPass<'d>;
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

impl Graphic for Triangle<f32> {
    fn draw<'d>(
        &'d self,
        draw_command: &'d mut DrawCommand,
        config: &'d DrawConfig,
    ) -> RenderPass<'d> {
        draw_command.draw_vertices(
            vec!(self.a, self.b, self.c),
            config
        )
    }
}
