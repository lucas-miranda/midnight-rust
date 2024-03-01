mod system_interface;
pub(crate) use system_interface::SystemInterface;

use crate::{ecs::component::BaseQuery, base::ApplicationState};

use super::FrameState;

pub trait System {
    type Query<'q>: BaseQuery;

    fn setup(&mut self);
    fn input<'q>(&mut self, query: Self::Query<'q>, state: &mut ApplicationState);
    fn run<'q>(&mut self, query: Self::Query<'q>, state: &mut FrameState);
    fn create_query<'q>(&self) -> Self::Query<'q>;
}
