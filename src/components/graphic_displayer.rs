use crate::{
    ecs::component::{
        Component,
        Components,
        ComponentAttribute,
    },
    rendering::{
        graphics::Graphic,
        ShaderConfig,
        TextureConfig,
        Vertex,
    },
};

#[derive(Default)]
pub struct GraphicDisplayer<V> where
    V: Vertex
{
    pub graphic: Option<Box<dyn Graphic<V>>>,
    pub shader_config: Option<ShaderConfig>,
    pub texture_config: Option<TextureConfig>,
}

impl<V: Vertex> Component for GraphicDisplayer<V> {
    fn attributes(&self) -> ComponentAttribute {
        ComponentAttribute::None
    }

    fn registered(&mut self, _components: &mut Components) {
    }

    fn unregistered(&mut self) {
    }
}
