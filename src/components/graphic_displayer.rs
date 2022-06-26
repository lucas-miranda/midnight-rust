use crate::{
    ecs::component::{Component, ComponentUnique},
    rendering::graphics::Graphic,
};

#[derive(Default)]
pub struct GraphicDisplayer {
    pub graphic: Option<Box<dyn Graphic>>,
}

impl GraphicDisplayer {
}

impl Component for GraphicDisplayer {
    fn as_unique(&self) -> Option<&dyn ComponentUnique> {
        None
    }
}
