use std::{
    any::Any,
    cell::RefCell,
    marker::PhantomData,
    ops::Deref,
    rc::{Rc, Weak},
};

use crate::ecs::EntityId;
use super::Component;

pub struct ComponentRef<C: Component> {
    entity_id: EntityId,
    weak: Weak<(dyn Any + 'static)>,
    strong: RefCell<Option<Rc<C>>>,
    phantom: PhantomData<C>,
}

impl<C: 'static + Component> ComponentRef<C> {
    pub fn new(entity_id: EntityId, weak: Weak<(dyn Any + 'static)>) -> Self {
        Self {
            entity_id,
            weak,
            strong: RefCell::new(None),
            phantom: PhantomData::default(),
        }
    }

    pub fn as_deref<'a>(&'a self) -> impl Deref<Target = Option<Rc<C>>> + 'a {
        if self.strong.borrow().is_none() {
            self.strong.replace(Some(self.weak.upgrade().unwrap().downcast().unwrap()));
        }

        self.strong.borrow()
    }

    pub fn entity_id(&self) -> EntityId {
        self.entity_id
    }
}
