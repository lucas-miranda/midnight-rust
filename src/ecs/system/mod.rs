mod system_interface;
pub use system_interface::SystemInterface;

use crate::ecs::component::BaseQuery;

use super::FrameState;

pub trait System {
    type Query<'q>: BaseQuery;

    fn setup(&mut self);
    fn input<'q>(&mut self, query: Self::Query<'q>, event: &winit::event::DeviceEvent);
    fn run<'q>(&mut self, query: Self::Query<'q>, state: &FrameState);
    fn create_query<'q>(&self) -> Self::Query<'q>;
}
