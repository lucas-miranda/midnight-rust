use std::{
    cell::RefCell,
    marker::PhantomData,
    rc::Rc,
};

use crate::ecs::component::{
    AnyComponent,
    Component,
    ComponentValueRef,
    ComponentValueMutRef,
};

pub struct ComponentStrongRef<'a, C: 'static + Component> {
    strong: Rc<RefCell<dyn AnyComponent>>,
    phantom: PhantomData<&'a C>,
}

impl<'a, C: 'static + Component> ComponentStrongRef<'a, C> {
    pub(crate) fn new(strong: Rc<RefCell<dyn AnyComponent>>) -> Self {
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
