use std::{
    //any::Any,
    cell::{RefCell, Ref, RefMut},
    marker::PhantomData,
    //ops::{Deref, DerefMut},
    rc::{Rc, Weak},
};

use crate::ecs::EntityId;
use super::{AnyComponent, Component};

pub struct ComponentRef<C: Component> {
    entity_id: EntityId,
    weak: Weak<RefCell<(dyn AnyComponent + 'static)>>,
    //strong: RefCell<Option<ComponentStrongRef<'a, C>>>,
    phantom: PhantomData<C>,
}

impl<C: 'static + Component> ComponentRef<C> {
    pub(super) fn new(
        entity_id: EntityId,
        weak: Weak<RefCell<(dyn AnyComponent + 'static)>>,
    ) -> Self {
        Self {
            entity_id,
            weak,
            //strong: RefCell::new(None),
            phantom: PhantomData::default(),
        }
    }

    pub fn retrieve(&self) -> Result<ComponentStrongRef<C>, &'static str> {
        match self.weak.upgrade() {
            Some(strong) => Ok(ComponentStrongRef::new(strong)),

            /*
            Some(strong) => match strong.downcast::<RefCell<C>>() {
                Ok(cmp_strong_ref) => Ok(ComponentStrongRef::new(cmp_strong_ref)),
                Err(_) => Err("Failed to downcast to component type"),
            },
            */
            None => Err("Can't upgrade from weak ref"),
        }
    }

    pub fn entity_id(&self) -> EntityId {
        self.entity_id
    }
}

pub struct ComponentStrongRef<'a, C: 'static + Component> {
    strong: Rc<RefCell<dyn AnyComponent>>,
    phantom: PhantomData<&'a C>,
}

impl<'a, C: 'static + Component> ComponentStrongRef<'a, C> {
    fn new(strong: Rc<RefCell<dyn AnyComponent>>) -> Self {
        Self {
            strong,
            phantom: Default::default(),
        }
    }

    pub fn borrow(&self) -> ComponentValueRef<C> {
        ComponentValueRef::new(self.strong.borrow())
    }

    pub fn borrow_mut(&self) -> ComponentValueMutRef<C> {
        ComponentValueMutRef::new(self.strong.borrow_mut())
    }
}

pub struct ComponentValueRef<'a, C> {
    value: Ref<'a, dyn AnyComponent>,
    phantom: PhantomData<C>,
}

impl<'a, C: 'static + Component> ComponentValueRef<'a, C> {
    fn new(value: Ref<'a, dyn AnyComponent>) -> Self {
        Self {
            value,
            phantom: PhantomData::default(),
        }
    }

    pub fn get_ref(&self) -> &C {
        self.value.as_any().downcast_ref().unwrap()
    }
}

pub struct ComponentValueMutRef<'a, C> {
    value: RefMut<'a, dyn AnyComponent>,
    phantom: PhantomData<C>,
}

impl<'a, C: 'static + Component> ComponentValueMutRef<'a, C> {
    fn new(value: RefMut<'a, dyn AnyComponent>) -> Self {
        Self {
            value,
            phantom: PhantomData::default(),
        }
    }

    pub fn get_ref(&self) -> &C {
        self.value.as_any().downcast_ref().unwrap()
    }

    pub fn get_mut_ref(&mut self) -> &mut C {
        self.value.as_any_mut().downcast_mut().unwrap()
    }
}

pub struct ComponentAnyRef {
    entity_id: EntityId,
    weak: Weak<RefCell<(dyn AnyComponent + 'static)>>,
}

impl ComponentAnyRef {
    pub(super) fn new(
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
