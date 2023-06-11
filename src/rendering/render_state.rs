use std::slice::Iter;
use super::{
    DrawConfig,
    Texture,
    Vertex,
};

pub trait RenderState<V> where
    V: Vertex,
{
    fn extend<'t>(
        &mut self,
        vertices: Iter<V>,
        texture: Option<&'t Texture>,
        draw_config: DrawConfig<V>,
    );
}
