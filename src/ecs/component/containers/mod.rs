mod component_container;
pub use component_container::ComponentContainer;

mod component_fn_container;
pub use component_fn_container::ComponentFnContainer;

use crate::ecs::component::{
    Component,
    Components,
};

pub trait ComponentHandlerContainer {
    type ComponentQuery;

    fn capture_components(&mut self, components: &Components);
}

impl<'a, T, C1, U, C2> ComponentHandlerContainer for (T, U) where
    C1: Component + 'static,
    C2: Component + 'static,
    T: ComponentHandlerContainer<ComponentQuery = C1>,
    U: ComponentHandlerContainer<ComponentQuery = C2>,
{
    type ComponentQuery = (C1, C2);

    fn capture_components(&mut self, components: &Components) {
        self.0.capture_components(components);
        self.1.capture_components(components);
    }
}
