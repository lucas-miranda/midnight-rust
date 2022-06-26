use std::{
    any::{Any, TypeId},
    collections::HashMap,
    rc::Rc,
};

use crate::ecs::EntityId;
use super::{
    Component,
    ComponentRef,
};

type ComponentEntry = Rc<(dyn Any + 'static)>;

pub struct Components {
    entity_id: EntityId,
    entries: Vec<ComponentEntry>,
    unique_entries: HashMap<TypeId, ComponentEntry>
}

impl Components {
    pub fn new(entity_id: EntityId) -> Self {
        Self {
            entity_id,
            entries: Vec::new(),
            unique_entries: HashMap::new(),
        }
    }

    pub fn count(&self) -> usize {
        self.entries.len()
    }

    pub fn register<C>(&mut self, component: C) -> Option<Rc<C>> where
        C: Component + 'static
    {
        /*
        println!(
            "registering component {} ({:?}) to entity {}",
            std::any::type_name::<C>(),
            std::any::TypeId::of::<C>(),
            self.entity_id,
        );
        */

        match component.as_unique() {
            Some(_unique) => {
                //println!("as unique");
                self.unique_entries.insert(TypeId::of::<C>(), Rc::new(component))
                    .map(|a| a.downcast().unwrap())
            },
            None => {
                //println!("as regular");
                self.entries.push(Rc::new(component));
                None
            },
        }
    }

    pub fn iter_kind<'c, C>(&'c self) -> impl Iterator<Item = ComponentRef<C>> + 'c where
        C: Component + 'static
    {
        self.unique_entries.values()
            .chain(self.entries.iter())
            .filter_map(|c| {
                if c.is::<C>() {
                    Some(ComponentRef::new(self.entity_id, Rc::downgrade(&c)))
                } else {
                    None
                }
            })
            .into_iter()

    }
}
