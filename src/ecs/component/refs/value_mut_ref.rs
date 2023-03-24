use std::{
    cell::RefMut,
    marker::PhantomData,
    ops::{Deref, DerefMut},
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
}

impl<'a, C: 'static + Component> Deref for ComponentValueMutRef<'a, C> {
    type Target = C;

    fn deref(&self) -> &Self::Target {
        self.value.as_any().downcast_ref().unwrap()
    }
}

impl<'a, C: 'static + Component> DerefMut for ComponentValueMutRef<'a, C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.value.as_any_mut().downcast_mut().unwrap()
    }
}
