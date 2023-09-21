mod triangle;

mod grid;
use std::any::Any;

pub use grid::Grid;

mod image;
pub use self::image::Image;

mod graphic_draw_error;
pub use graphic_draw_error::GraphicDrawError;

mod text;
pub use text::Text;

use super::{
    DrawConfig,
    RenderState,
    Texture,
    Vertex,
};

pub trait Graphic<V> : Any where
    V: Vertex
{
    fn texture<'t>(&'t self) -> Option<&'t Texture>;

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn draw<'d>(
        &'d self,
        state: &'d mut dyn RenderState<V>,
        draw_config: DrawConfig<V>,
    ) -> Result<(), GraphicDrawError>;
}


