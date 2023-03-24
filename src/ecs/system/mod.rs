mod system_interface;
pub use system_interface::SystemInterface;

use crate::ecs::component::{
    Component,
    ComponentHandlerContainer,
};

pub trait System {
    type Component: Component;
    type Container: ComponentHandlerContainer;

    fn setup(&mut self);
    fn run(&mut self, query: <Self::Container as ComponentHandlerContainer>::Query);
    fn create_container(&self) -> Self::Container;
}
