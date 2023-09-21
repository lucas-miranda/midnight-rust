use std::collections::BTreeMap;

use crate::ecs::{
    component::{
        AnyComponent,
        ComponentQueryIterator,
        ComponentStrongRef,
        ComponentValueMutRef,
        ComponentValueRef,
        Components,
    },
    entity::EntityId,
};

use super::{
    BaseQuery,
    QueryEntry,
};

/// Multiple entry query, but only one `Component` per `Entity`.
/// Uses component type to filter it's entries.
///
/// [`Component`]: crate::ecs::components::Component
/// [`Entry`]: crate::ecs::entity::Entry
pub struct Query<'a, C: 'static + AnyComponent> {
    container: BTreeMap<EntityId, ComponentStrongRef<'a, C>>,
}

impl<'a, C: 'static + AnyComponent> Default for Query<'a, C> {
    fn default() -> Self {
        Self {
            container: BTreeMap::default(),
        }
    }
}

impl<'a, C: 'static + AnyComponent> Query<'a, C> {
    /// Retrieve first component found
    pub fn component<'r>(
        &'r self
    ) -> Result<QueryEntry<<Self as BaseQuery>::Target<'r>>, &'static str> {
        for element in self.container.iter().take(1) {
            return Ok(QueryEntry::new(
                element.0.clone(),
                element.1.borrow(),
            ))
        }

        Err("Not found")
    }

    pub fn component_mut<'r>(
        &'r self
    ) -> Result<QueryEntry<ComponentValueMutRef<'r, C>>, &'static str> {
        for element in self.container.iter().take(1) {
            return Ok(QueryEntry::new(
                element.0.clone(),
                element.1.borrow_mut(),
            ))
        }

        Err("Not found")
    }

    pub fn count(&self) -> usize {
        self.container.len()
    }
}

impl<'a, C: 'static + AnyComponent> BaseQuery for Query<'a, C> {
    type Target<'t> = ComponentValueRef<'t, C> where Self: 't;

    fn capture_components(&mut self, components: &Components) {
        for component in components.iter_kind::<C>() {
            self.container.insert(*components.entity_id(), component.consume().unwrap());
            break;
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

//
//
//

pub struct MutQuery<'a, C: 'static + AnyComponent> {
    container: BTreeMap<EntityId, ComponentStrongRef<'a, C>>,
}


impl<'a, C: 'static + AnyComponent> Default for MutQuery<'a, C> {
    fn default() -> Self {
        Self {
            container: BTreeMap::default(),
        }
    }
}

impl<'a, C: 'static + AnyComponent> MutQuery<'a, C> {
    /// Retrieve first component found
    pub fn component<'r>(
        &'r self
    ) -> Result<QueryEntry<<Self as BaseQuery>::Target<'r>>, &'static str> {
        for element in self.container.iter().take(1) {
            return Ok(QueryEntry::new(
                element.0.clone(),
                element.1.borrow_mut(),
            ))
        }

        Err("Not found")
    }

    pub fn component_mut<'r>(
        &'r self
    ) -> Result<QueryEntry<ComponentValueMutRef<'r, C>>, &'static str> {
        for element in self.container.iter().take(1) {
            return Ok(QueryEntry::new(
                element.0.clone(),
                element.1.borrow_mut(),
            ))
        }

        Err("Not found")
    }

    pub fn count(&self) -> usize {
        self.container.len()
    }
}

impl<'a, C: 'static + AnyComponent> BaseQuery for MutQuery<'a, C> {
    type Target<'t> = ComponentValueMutRef<'t, C> where Self: 't;

    fn capture_components(&mut self, components: &Components) {
        for component in components.iter_kind::<C>() {
            self.container.insert(*components.entity_id(), component.consume().unwrap());
            break;
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
                    c.1.borrow_mut(),
                ))
        )
    }
}
