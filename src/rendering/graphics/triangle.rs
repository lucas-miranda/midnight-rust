use crate::{
    math::{
        Triangle,
        num_traits::Num,
        Vector2,
    },
    rendering::VertexPosition,
};

use super::{
    Graphic,
    DrawConfig,
    RenderState,
    Texture,
};

impl<D: Num + Copy + Clone, V: VertexPosition<Position = Vector2<D>>> Graphic<V> for Triangle<D> {
    fn texture<'t>(&'t self) -> Option<&'t Texture> {
        None
    }

    fn draw<'d>(
        &'d self,
        state: &'d mut dyn RenderState<V>,
        draw_config: DrawConfig<V>,
    ) {
        state.extend(
            vec!(
                V::from_position(self.a),
                V::from_position(self.b),
                V::from_position(self.c),
            ).iter(),
            None,
            draw_config,
        )
    }
}
