use std::{
    cell::RefCell,
    marker::PhantomData,
    rc::Weak,
};

use crate::ecs::{
    component::{
        AnyComponent,
        Component,
        ComponentStrongRef,
    },
    entity::EntityId,
};

pub struct ComponentRef<C: Component> {
    entity_id: EntityId,
    weak: Weak<RefCell<(dyn AnyComponent + 'static)>>,
    phantom: PhantomData<C>,
}

impl<C: 'static + Component> ComponentRef<C> {
    pub(in crate::ecs::component) fn new(
        entity_id: EntityId,
        weak: Weak<RefCell<(dyn AnyComponent + 'static)>>,
    ) -> Self {
        Self {
            entity_id,
            weak,
            phantom: PhantomData::default(),
        }
    }

    pub fn retrieve(&self) -> Result<ComponentStrongRef<C>, &'static str> {
        match self.weak.upgrade() {
            Some(strong) => Ok(ComponentStrongRef::new(strong)),
            None => Err("Can't upgrade from weak ref"),
        }
    }

    pub fn entity_id(&self) -> EntityId {
        self.entity_id
    }
}
