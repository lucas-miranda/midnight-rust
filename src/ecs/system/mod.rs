mod system_interface;
pub use system_interface::SystemInterface;

use crate::ecs::component::ComponentHandlerContainer;

pub trait System {
    type Container: ComponentHandlerContainer + 'static;

    fn setup(&mut self);
    fn run(&mut self, container: &mut Self::Container);
    fn create_container(&self) -> Self::Container;
}
