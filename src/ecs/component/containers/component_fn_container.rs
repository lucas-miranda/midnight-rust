use std::ops::Deref;

use crate::ecs::component::{
    AnyComponent,
    Components,
    ComponentStrongAnyRef,
};

use super::ComponentHandlerContainer;

pub struct ComponentFnContainer {
    container: Vec<ComponentStrongAnyRef>,
    filter: Box<dyn Fn(&ComponentStrongAnyRef) -> bool>,
}

impl ComponentFnContainer {
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

impl ComponentHandlerContainer for ComponentFnContainer {
    type ComponentQuery = Box<dyn AnyComponent>;

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
}

impl Deref for ComponentFnContainer {
    type Target = [ComponentStrongAnyRef];

    fn deref(&self) -> &Self::Target {
        &self.container
    }
}
