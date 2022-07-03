use crate::{
    ecs::component::{
        AnyComponent,
        Component,
        Components,
        ComponentUnique,
        ComponentUpdatable,
    },
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

    fn as_unique_mut(&mut self) -> Option<&mut dyn ComponentUnique> {
        None
    }

    fn as_updatable(&self) -> Option<&dyn ComponentUpdatable> {
        None
    }

    fn as_updatable_mut(&mut self) -> Option<&mut dyn ComponentUpdatable> {
        None
    }

    fn registered(&mut self, components: &mut Components) {
    }

    fn unregistered(&mut self) {
    }
}

impl AnyComponent for GraphicDisplayer {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn as_component(&self) -> &dyn Component {
        self
    }

    fn as_component_mut(&mut self) -> &mut dyn Component {
        self
    }
}
