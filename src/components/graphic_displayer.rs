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
    // TODO  remove pub
    pub graphic: Option<Box<dyn Graphic<V>>>,
    pub shader_config: Option<ShaderConfig>,
    pub texture_config: Option<TextureConfig>,
}

impl<V: Vertex> GraphicDisplayer<V> {
    pub fn empty() -> Self {
        Self {
            graphic: None,
            shader_config: Default::default(),
            texture_config: Default::default(),
        }
    }

    pub fn new<G>(graphic: G) -> Self where
        G: Graphic<V>
    {
        Self {
            graphic: Some(Box::new(graphic)),
            shader_config: Default::default(),
            texture_config: Default::default(),
        }
    }

    pub fn retrieve_graphic<G>(&self) -> Option<&G> where
        G: Graphic<V>
    {
        self.graphic
            .as_ref()
            .and_then(|g| g.as_any().downcast_ref())
    }

    pub fn mut_retrieve_graphic<G>(&mut self) -> Option<&mut G> where
        G: Graphic<V>
    {
        self.graphic
            .as_mut()
            .and_then(|g| g.as_any_mut().downcast_mut())
    }

    pub fn replace_graphic<G>(&mut self, graphic: G) -> Option<Box<dyn Graphic<V>>> where
        G: Graphic<V>
    {
        self.graphic.replace(Box::new(graphic))
    }
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
