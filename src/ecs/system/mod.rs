mod system_interface;
pub use system_interface::SystemInterface;

use crate::ecs::component::ComponentQuery;

pub trait System {
    type Query<'q>: ComponentQuery;

    fn setup(&mut self);
    fn input<'q>(&mut self, query: Self::Query<'q>, event: &winit::event::DeviceEvent);
    fn run<'q>(&mut self, query: Self::Query<'q>);
    fn create_query<'q>(&self) -> Self::Query<'q>;
}
