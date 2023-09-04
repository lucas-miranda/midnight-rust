use std::slice::Iter;
use super::{
    DrawConfig,
    Texture,
    Vertex, RenderStateError,
};

/// Anything which receives vertices and any data related,
/// using it to render something later.
pub trait RenderState<V> where
    V: Vertex,
{
    fn extend<'t>(
        &mut self,
        vertices: Iter<V>,
        texture: Option<&'t Texture>,
        draw_config: DrawConfig<V>,
    ) -> Result<(), RenderStateError>;
}
