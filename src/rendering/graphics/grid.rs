use std::cell::RefMut;

use crate::{
    rendering::{
        //graphics::DrawCommand,
        DrawConfig,
        GraphicAdapter,
    },
    math::Triangle,
    util::Size,
};

use super::Graphic;

pub struct Grid {
    pub columns: u32,
    pub rows: u32,
    pub tile_size: Size<u32>,
}

impl Grid {
}

/*
impl Graphic for Grid {
    fn draw<'d>(
        &'d self,
        graphic_adapter: &'d mut RefMut<'_, GraphicAdapter>,
        config: &'d DrawConfig,
    ) -> DrawCommand<'d> {
        graphic_adapter.draw_vertices(
            vec!(self.a, self.b, self.c),
            config
        )
    }
}
*/
