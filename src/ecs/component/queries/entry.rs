use std::ops::{Deref, DerefMut};

use crate::ecs::entity::EntityId;

/// Entry result from a `BaseQuery`.
/// Component type is determined by `BaseQuery::Target`.
///
/// [`BaseQuery`]: super::BaseQuery
/// [`BaseQuery::Target`]: super::BaseQuery::Target
pub struct QueryEntry<C> {
    entity_id: EntityId,

    // a good reason to keep pub instead using getters:
    //  - tuple desconstruct when C is (A, B) or larger,
    //    it's way better to write:
    //      QueryEntry { component: (a, b), .. } = entry;
    //    than:
    //      let component = entry.component();
    //      let a = component.0;
    //      let b = component.1;
    pub component: C,
}

impl<C> QueryEntry<C> {
    pub(super) fn new(entity_id: EntityId, component: C) -> Self {
        Self {
            entity_id,
            component,
        }
    }

    /// Entity Id which owns queried component.
    pub fn entity_id(&self) -> &EntityId {
        &self.entity_id
    }
}

impl<C> Deref for QueryEntry<C> {
    type Target = C;

    fn deref(&self) -> &Self::Target {
        &self.component
    }
}

impl<C> DerefMut for QueryEntry<C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.component
    }
}
