mod triangle;

mod grid;
pub use grid::Grid;

mod image;
pub use self::image::Image;

use super::{
    DrawConfig,
    RenderState,
    Texture,
    Vertex,
};

pub trait Graphic<V> where
    V: Vertex
{
    fn texture<'t>(&'t self) -> Option<&'t Texture>;

    fn draw<'d>(
        &'d self,
        state: &'d mut dyn RenderState<V>,
        draw_config: DrawConfig<V>,
    );
}
