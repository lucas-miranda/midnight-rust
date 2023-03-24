mod component_container;
pub use component_container::ComponentContainer;

mod component_fn_container;
pub use component_fn_container::ComponentFnContainer;

use crate::ecs::component::{
    Components,
    ComponentQuery,
};

pub trait ComponentHandlerContainer {
    type Query: ComponentQuery;

    fn capture_components(&mut self, components: &Components);
    fn query(self) -> Self::Query;
}
