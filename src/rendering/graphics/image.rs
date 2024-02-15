use std::{marker::PhantomData, any::Any, rc::{Weak, Rc}};

use crate::{
    math::{Vector2, Rectangle, Size2},
    rendering::{
        VertexPosition,
        VertexTexture2D,
    },
    util::Size,
};

use super::{
    DrawConfig,
    Graphic,
    GraphicDrawError,
    RenderState,
    Texture,
};

pub struct Image<V: VertexPosition<Position = Vector2<f32>>> {
    texture: Weak<Texture>,
    phantom: PhantomData<V>,
    clip_region: Rectangle<i32>,
}

impl<V: VertexPosition<Position = Vector2<f32>> + VertexTexture2D> Image<V> {
    pub fn new(texture: Rc<Texture>) -> Self {
        Self {
            texture: Rc::downgrade(&texture),
            phantom: Default::default(),
            clip_region: Rectangle::new(Vector2::zero(), Size2::new(texture.width() as i32, texture.height() as i32))
        }
    }
}

impl<V: VertexPosition<Position = Vector2<f32>> + VertexTexture2D> Graphic<V> for Image<V> {
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
        let size: Size<f32>
            = Size::with(self.clip_region.size.width, self.clip_region.size.height).unwrap();

        //println!("image size: {}", size);
        let vertices = vec![
            V::from_position(Vector2::new(0.0, 0.0)).with_uv(Vector2::new(0.0, 0.0)),
            V::from_position(Vector2::new(size.width, 0.0)).with_uv(Vector2::new(1.0, 0.0)),
            V::from_position(Vector2::new(0.0, size.height)).with_uv(Vector2::new(0.0, 1.0)),

            V::from_position(Vector2::new(0.0, size.height)).with_uv(Vector2::new(0.0, 1.0)),
            V::from_position(Vector2::new(size.width, 0.0)).with_uv(Vector2::new(1.0, 0.0)),
            V::from_position(Vector2::new(size.width, size.height)).with_uv(Vector2::new(1.0, 1.0)),
        ];

        state.extend(vertices.iter(), Some(self.texture.clone()), draw_config)
             .map_err(GraphicDrawError::from)?;

        Ok(())
    }
}
