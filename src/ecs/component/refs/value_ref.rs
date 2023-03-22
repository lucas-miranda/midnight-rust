use std::{
    cell::Ref,
    marker::PhantomData,
};

use crate::ecs::component::{
    AnyComponent,
    Component,
};

pub struct ComponentValueRef<'a, C> {
    value: Ref<'a, dyn AnyComponent>,
    phantom: PhantomData<C>,
}

impl<'a, C: 'static + Component> ComponentValueRef<'a, C> {
    pub(super) fn new(value: Ref<'a, dyn AnyComponent>) -> Self {
        Self {
            value,
            phantom: PhantomData::default(),
        }
    }

    pub fn get_ref(&self) -> &C {
        self.value.as_any().downcast_ref().unwrap()
    }
}
