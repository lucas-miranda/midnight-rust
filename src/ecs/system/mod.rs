mod system_interface;
pub use system_interface::SystemInterface;

use crate::ecs::component::ComponentHandlerContainer;

pub trait System {
    type Container: ComponentHandlerContainer;

    fn setup(&mut self);
    fn input(&mut self, container: Self::Container, event: &winit::event::DeviceEvent);
    fn run(&mut self, container: Self::Container);
    fn create_container(&self) -> Self::Container;
}
