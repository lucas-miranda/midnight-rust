mod system_interface;
pub use system_interface::SystemInterface;

use crate::ecs::component::BaseQuery;

pub trait System {
    type Query<'q>: BaseQuery;

    fn setup(&mut self);
    fn input<'q>(&mut self, query: Self::Query<'q>, event: &winit::event::DeviceEvent);
    fn run<'q>(&mut self, query: Self::Query<'q>);
    fn create_query<'q>(&self) -> Self::Query<'q>;
}
