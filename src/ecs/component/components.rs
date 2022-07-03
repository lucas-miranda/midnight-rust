use std::{
    any::TypeId,
    cell::RefCell,
    collections::HashMap,
    rc::Weak,
};

use crate::ecs::entity::{Entity, EntityId};
use super::{
    AnyComponent,
    ComponentRef,
    ComponentEntry,
};

// TODO  make it a strong type around Rc<_>,
//       it'll just be a helper pointer wrapper type
//       it should be able to deref into Rc<_>
//type ComponentEntry = Rc<(dyn Any + 'static)>;

pub struct Components {
    //entity_id: EntityId,
    entity: Weak<RefCell<Entity>>,
    entries: Vec<ComponentEntry>,
    unique_entries: HashMap<TypeId, ComponentEntry>
}

impl Components {
    pub(crate) fn new(entity: Weak<RefCell<Entity>>) -> Self {
        Self {
            entity,
            entries: Vec::new(),
            unique_entries: HashMap::new(),
        }
    }

    pub fn count(&self) -> usize {
        self.entries.len()
    }

    pub fn register<C>(&mut self, mut component: C) -> Option<C> where
        C: AnyComponent + 'static
    {
        let entity_id = self.entity
            .upgrade()
            .unwrap()
            .borrow()
            .id();

        self.internal_register(component, entity_id)
    }

    pub fn iter(&self) -> impl Iterator<Item = &ComponentEntry> {
        self.unique_entries.values()
            .chain(self.entries.iter())
            .into_iter()
    }

    // TODO  maybe change to iter_ref_kind()
    pub fn iter_kind<'c, C>(&'c self) -> impl Iterator<Item = ComponentRef<C>> + 'c where
        C: AnyComponent + 'static
    {
        self.unique_entries.values()
            .chain(self.entries.iter())
            .filter_map(|c| match c.is::<C>() {
                true => Some(c.get_ref()),
                false => None,
            })
            .into_iter()

    }

    /// Get first component with matches provided type.
    pub fn get_kind<C>(&self) -> Option<ComponentRef<C>> where
        C: AnyComponent + 'static
    {
        self.unique_entries
            .values()
            .chain(self.entries.iter())
            .find_map(|c| match c.is::<C>() {
                true => Some(c.get_ref()),
                false => None,
            })
    }

    pub(in crate::ecs) fn internal_register<C>(
        &mut self,
        mut component: C,
        entity_id: EntityId,
    ) -> Option<C> where
        C: AnyComponent + 'static
    {
        component.registered(self);

        match component.as_unique() {
            Some(_unique) => {
                // as unique
                self.unique_entries.insert(
                        TypeId::of::<C>(),
                        ComponentEntry::new(entity_id, component)
                    );
                    //.map(|a| a.leak::<C>())
                None
            },
            None => {
                // as regular
                self.entries.push(ComponentEntry::new(entity_id, component));
                None
            },
        }
    }
}
