mod system_interface;
pub use system_interface::SystemInterface;

use crate::ecs::component::ComponentQuery;

pub trait System {
    type Query: ComponentQuery;

    fn setup(&mut self);
    fn input(&mut self, query: Self::Query, event: &winit::event::DeviceEvent);
    fn run(&mut self, query: Self::Query);
    fn create_query(&self) -> Self::Query;
}
