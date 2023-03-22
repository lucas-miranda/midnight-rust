use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use crate::ecs::component::{
    Components,
    ComponentAnyRef,
    AnyComponent,
};

use super::ComponentHandlerContainer;

pub struct ComponentFnContainer {
    container: Vec<ComponentAnyRef>,
    filter: Box<dyn Fn(Rc<RefCell<dyn AnyComponent>>) -> bool>,
}

impl ComponentFnContainer {
    pub fn new<F: 'static + Fn(Rc<RefCell<dyn AnyComponent>>) -> bool>(filter: F) -> Self {
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
    fn register_components(&mut self, components: &Components) {
        for entry in components.iter() {
            let reference = entry.get_any_ref();

            match reference.retrieve() {
                Ok(strong_ref) => {
                    if (self.filter)(strong_ref) {
                        self.container.push(reference);
                    }

                    /*
                    // register only updatable components
                    if strong_ref.as_updatable().is_some() {
                        self.container.push(reference);
                    }
                    */
                },
                Err(_) => (),
            }
        }
    }
}

impl Deref for ComponentFnContainer {
    type Target = [ComponentAnyRef];

    fn deref(&self) -> &Self::Target {
        &self.container
    }
}
