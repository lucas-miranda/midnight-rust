use std::any::Any;

use crate::{
    math::{ num_traits::Num, Vector2, Rectangle },
    rendering::VertexPosition,
};

use super::{
    DrawConfig,
    Graphic,
    GraphicDrawError,
    RenderState,
};

impl<D, V> Graphic<V> for Rectangle<D> where
    D: Num + Copy + Clone + 'static,
    V: VertexPosition<Position = Vector2<D>>,
{
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
                V::from_position(self.position),
                V::from_position(self.position + Vector2::new(self.size.width, D::zero())),
                V::from_position(self.position + Vector2::new(D::zero(), self.size.height)),

                V::from_position(self.position + Vector2::new(D::zero(), self.size.height)),
                V::from_position(self.position + Vector2::new(self.size.width, D::zero())),
                V::from_position(self.position + Vector2::new(self.size.width, self.size.height)),
            ).iter(),
            None,
            draw_config,
        ).map_err(GraphicDrawError::from)
    }
}
