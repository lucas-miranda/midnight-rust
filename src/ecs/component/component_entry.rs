use std::{
    any::Any,
    cell::RefCell,
    rc::Rc,
};

use crate::ecs::entity::EntityId;
use super::{
    AnyComponent,
    Component,
    ComponentRef,
    ComponentAnyRef,
};

pub type RawComponentEntry = Rc<RefCell<(dyn AnyComponent + 'static)>>;

/// Describes a component entry at Components
pub struct ComponentEntry {
    entity_id: EntityId,
    component: RawComponentEntry,
}

impl ComponentEntry {
    pub fn new<C>(entity_id: EntityId, component: C) -> Self where
        C: AnyComponent + 'static
    {
        Self {
            entity_id,
            component: Rc::new(RefCell::new(component)),
        }
    }

    pub fn get_ref<C>(&self) -> ComponentRef<C> where
        C: Component + 'static
    {
        ComponentRef::new(self.entity_id, Rc::downgrade(&self.component))
    }

    pub fn get_any_ref(&self) -> ComponentAnyRef {
        ComponentAnyRef::new(self.entity_id, Rc::downgrade(&self.component))
    }

    pub fn leak<C>(self) -> C where
        C: AnyComponent + 'static
    {
        let b: Box<dyn Any> = Box::new(self.component);
        let c = b.downcast::<Rc<RefCell<C>>>()
                 .unwrap();

        match Rc::try_unwrap(*c) {
            Ok(a) => a.into_inner(),
            Err(_) => panic!("Failed to leak"),
        }
    }

    pub fn inner(&self) -> &RawComponentEntry {
        &self.component
    }

    pub fn is<C>(&self) -> bool where
        C: AnyComponent + 'static
    {
        self.component.borrow().as_any().is::<C>()
    }
}
