use std::{
    cell,
    ops::Deref,
};

use crate::ecs::component::{
    AnyComponent,
    Components,
    ComponentQueryIterator,
    ComponentStrongAnyRef,
};

use super::ComponentQuery;

pub struct FnQuery {
    container: Vec<ComponentStrongAnyRef>,
    filter: Box<dyn Fn(&ComponentStrongAnyRef) -> bool>,
}

impl FnQuery {
    pub fn new<F: 'static + Fn(&ComponentStrongAnyRef) -> bool>(filter: F) -> Self {
        Self {
            container: Vec::new(),
            filter: Box::new(filter),
        }
    }

    pub fn count(&self) -> usize {
        self.container.len()
    }
}

impl ComponentQuery for FnQuery {
    type Target<'t> = cell::Ref<'t, dyn AnyComponent> where Self : 't;

    fn capture_components(&mut self, components: &Components) {
        for entry in components.iter() {
            let reference = entry.get_any_ref();

            match reference.consume() {
                Ok(strong_ref) => {
                    if (self.filter)(&strong_ref) {
                        self.container.push(strong_ref);
                    }
                },
                Err(_) => (),
            }
        }
    }

    fn iter_components<'i>(&'i self) -> ComponentQueryIterator<'i, Self::Target<'i>> {
        fn convert(c: &ComponentStrongAnyRef) -> cell::Ref<'_, dyn AnyComponent> {
            c.borrow()
        }

        ComponentQueryIterator::new(
            self.container
                .iter()
                .map(convert)
        )
    }
}

impl Deref for FnQuery {
    type Target = [ComponentStrongAnyRef];

    fn deref(&self) -> &Self::Target {
        &self.container
    }
}
