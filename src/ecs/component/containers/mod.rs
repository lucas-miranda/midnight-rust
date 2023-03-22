mod component_container;
pub use component_container::ComponentContainer;

mod component_fn_container;
pub use component_fn_container::ComponentFnContainer;


use crate::ecs::component::Components;

pub trait ComponentHandlerContainer {
    fn register_components(&mut self, components: &Components);
}
