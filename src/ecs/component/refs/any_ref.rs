use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::ecs::{
    component::AnyComponent,
    entity::EntityId,
};

pub struct ComponentAnyRef {
    entity_id: EntityId,
    weak: Weak<RefCell<(dyn AnyComponent + 'static)>>,
}

impl ComponentAnyRef {
    pub(in crate::ecs::component) fn new(
        entity_id: EntityId,
        weak: Weak<RefCell<(dyn AnyComponent + 'static)>>,
    ) -> Self {
        Self {
            entity_id,
            weak,
        }
    }

    pub fn retrieve(&self) -> Result<Rc<RefCell<dyn AnyComponent>>, &'static str> {
        match self.weak.upgrade() {
            Some(strong) => Ok(strong),
            None => Err("Can't upgrade from weak ref"),
        }
    }
}
