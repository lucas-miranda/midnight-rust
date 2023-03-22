use std::{
    cell::RefMut,
    marker::PhantomData,
};

use crate::ecs::component::{
    AnyComponent,
    Component,
};

pub struct ComponentValueMutRef<'a, C> {
    value: RefMut<'a, dyn AnyComponent>,
    phantom: PhantomData<C>,
}

impl<'a, C: 'static + Component> ComponentValueMutRef<'a, C> {
    pub(super) fn new(value: RefMut<'a, dyn AnyComponent>) -> Self {
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
