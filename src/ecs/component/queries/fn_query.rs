use std::{
    cell,
    collections::BTreeMap,
};

use crate::ecs::{
    component::{
        AnyComponent,
        Components,
        ComponentQueryIterator,
        ComponentStrongAnyRef, QueryEntry,
    },
    entity::EntityId,
};

use super::BaseQuery;

/// Multiple entry query, but only one `Component` per `Entity`.
/// Instead relying on `Component` type, uses a filter to determine entries, allowing components
/// with different types.
///
/// [`Component`]: crate::ecs::components::Component
/// [`Entry`]: crate::ecs::entity::Entry
pub struct FnQuery {
    container: BTreeMap<EntityId, ComponentStrongAnyRef>,
    filter: Box<dyn Fn(&ComponentStrongAnyRef) -> bool>,
}

impl FnQuery {
    pub fn new<F: 'static + Fn(&ComponentStrongAnyRef) -> bool>(filter: F) -> Self {
        Self {
            container: BTreeMap::default(),
            filter: Box::new(filter),
        }
    }

    pub fn count(&self) -> usize {
        self.container.len()
    }
}

impl BaseQuery for FnQuery {
    type Target<'t> = cell::Ref<'t, dyn AnyComponent> where Self : 't;

    fn capture_components(&mut self, components: &Components) {
        for entry in components.iter() {
            let reference = entry.get_any_ref();

            match reference.consume() {
                Ok(strong_ref) => {
                    if (self.filter)(&strong_ref) {
                        self.container.insert(*components.entity_id(), strong_ref);
                    }
                },
                Err(_) => (),
            }
        }
    }

    fn iter_components<'i>(
        &'i self
    ) -> ComponentQueryIterator<'i, QueryEntry<Self::Target<'i>>> {
        ComponentQueryIterator::new(
            self.container
                .iter()
                .map(|c| QueryEntry::new(
                    c.0.clone(),
                    c.1.borrow(),
                ))
        )
    }
}
