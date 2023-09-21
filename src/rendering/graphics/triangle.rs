use std::any::Any;

use crate::{
    math::{
        num_traits::Num,
        Triangle,
        Vector2,
    },
    rendering::VertexPosition,
};

use super::{
    DrawConfig,
    Graphic,
    GraphicDrawError,
    RenderState,
    Texture,
};

impl<D, V> Graphic<V> for Triangle<D> where
    D: Num + Copy + Clone + 'static,
    V: VertexPosition<Position = Vector2<D>>,
{
    fn texture<'t>(&'t self) -> Option<&'t Texture> {
        None
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn draw<'d>(
        &'d self,
        state: &'d mut dyn RenderState<V>,
        draw_config: DrawConfig<V>,
    ) -> Result<(), GraphicDrawError> {
        state.extend(
            vec!(
                V::from_position(self.a),
                V::from_position(self.b),
                V::from_position(self.c),
            ).iter(),
            None,
            draw_config,
        ).map_err(GraphicDrawError::from)
    }
}
