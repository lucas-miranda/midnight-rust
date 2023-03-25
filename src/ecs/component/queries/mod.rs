mod query;
pub use query::Query;

mod fn_query;
pub use fn_query::FnQuery;

use crate::ecs::component::{
    Component,
    Components,
};

pub trait ComponentQuery {
    type Target;

    fn capture_components(&mut self, components: &Components);
}

impl<'a, T, C1, U, C2> ComponentQuery for (T, U) where
    C1: Component + 'static,
    C2: Component + 'static,
    T: ComponentQuery<Target = C1>,
    U: ComponentQuery<Target = C2>,
{
    type Target = (C1, C2);

    fn capture_components(&mut self, components: &Components) {
        self.0.capture_components(components);
        self.1.capture_components(components);
    }
}
