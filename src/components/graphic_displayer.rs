use crate::{
    ecs::component::{
        Component,
        Components,
        ComponentAttribute,
    },
    rendering::{
        graphics::Graphic,
        shaders::ShaderId,
    },
};

#[derive(Default)]
pub struct GraphicDisplayer {
    pub graphic: Option<Box<dyn Graphic>>,
    pub shader_id: Option<ShaderId>,
}

impl Component for GraphicDisplayer {
    fn attributes(&self) -> ComponentAttribute {
        ComponentAttribute::None
    }

    fn registered(&mut self, _components: &mut Components) {
    }

    fn unregistered(&mut self) {
    }
}
