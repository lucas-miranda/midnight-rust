mod query;

pub use query::Query;

mod fn_query;
pub use fn_query::FnQuery;

mod unit_query;
pub use unit_query::UnitQuery;

/*
mod compound_query;
pub use compound_query::CompoundQuery;
*/

mod iterator;
pub use iterator::ComponentQueryIterator;

use crate::ecs::component::Components;

pub trait ComponentQuery {
    type Target<'t> where Self: 't;

    fn capture_components(&mut self, components: &Components);
    fn iter_components<'i>(&'i self) -> ComponentQueryIterator<'i, Self::Target<'i>>;
}


impl<'a, T, U> ComponentQuery for (T, U) where
    T: ComponentQuery,
    U: ComponentQuery,
{
    type Target<'t> = (T::Target<'t>, U::Target<'t>) where T: 't, U: 't;

    fn capture_components(&mut self, components: &Components) {
        self.0.capture_components(components);
        self.1.capture_components(components);
    }

    fn iter_components<'i>(&'i self) -> ComponentQueryIterator<'i, Self::Target<'i>> {
        ComponentQueryIterator::new(
            self.0.iter_components()
                .zip(self.1.iter_components())
        )
    }
}
