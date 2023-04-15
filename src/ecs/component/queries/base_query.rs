use crate::ecs::component::Components;

use super::{
    ComponentQueryIterator,
    QueryEntry,
};

/// Describes a `System` component query.
/// It'll run at `System` registered `Domain`.
///
/// [`System`]: crate::ecs::System
/// [`System`]: crate::ecs::Domain
pub trait BaseQuery {
    type Target<'t> where Self: 't;

    /// Capture `Component` using query conditions.
    /// Zero or more `Component` can be captured.
    ///
    /// [`Component`]: crate::ecs::components::Component
    fn capture_components(&mut self, components: &Components);

    /// Returns a `ComponentQueryIterator` which works as an interface to access query results.
    ///
    /// [`ComponentQueryIterator`]: super::ComponentQueryIterator
    fn iter_components<'i>(
        &'i self
    ) -> ComponentQueryIterator<'i, QueryEntry<Self::Target<'i>>>;
}


impl<'a, T, U> BaseQuery for (T, U) where
    T: BaseQuery,
    U: BaseQuery,
{
    type Target<'t> = (Option<T::Target<'t>>, Option<U::Target<'t>>) where T: 't, U: 't;

    fn capture_components(&mut self, components: &Components) {
        self.0.capture_components(components);
        self.1.capture_components(components);
    }

    fn iter_components<'i>(
        &'i self
    ) -> ComponentQueryIterator<'i, QueryEntry<Self::Target<'i>>> {
        ComponentQueryIterator::new(
            self.0.iter_components()
            .map(|entry_a| {
                for entry_b in self.1.iter_components() {
                    if entry_b.entity_id().eq(&entry_a.entity_id()) {
                        return QueryEntry::new(
                            entry_a.entity_id().clone(),
                            (Some(entry_a.component), Some(entry_b.component)),
                        );
                    }
                }

                return QueryEntry::new(
                    entry_a.entity_id().clone(),
                    (Some(entry_a.component), None),
                );
            })
        )
    }
}
