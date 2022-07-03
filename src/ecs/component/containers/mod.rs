use std::cell::RefCell;
use std::{ops::Deref, marker::PhantomData};
use std::rc::Rc;

use crate::ecs::component::{
    Component,
    Components,
    ComponentRef,
    ComponentStrongRef,
    ComponentAnyRef,
    AnyComponent,
};

pub trait ComponentHandlerContainer {
    fn register_components(&mut self, components: &Components);
}

//

/*
pub trait ComponentAccessDescriptor<C: Component> {
}

//

pub struct ImmutableAccess<C: Component> {
    phantom_component: PhantomData<C>,
}

impl<C: Component> ComponentAccessDescriptor<C> for ImmutableAccess<C> {
}
*/

//

#[derive(Default)]
pub struct ComponentContainer<C: AnyComponent> {
    container: Vec<ComponentRef<C>>,
}

impl<C: 'static + AnyComponent> ComponentContainer<C> {
    /// Retrieve first component found
    pub fn component<'a>(&'a self) -> Result<ComponentStrongRef<C>, &'static str> {
        if let Some(element) = self.container.first() {
            return element.retrieve()
        }

        Err("Not found")
    }

    /// Retrieve a `ComponentRef` to the first component found
    pub fn component_ref(&self) -> Option<&ComponentRef<C>> {
        if let Some(element) = self.container.first() {
            return Some(&element);
        }

        None
    }

    pub fn register(&mut self, component: ComponentRef<C>) {
        self.container.push(component);
    }

    pub fn count(&self) -> usize {
        self.container.len()
    }

    /*
    pub fn iter(&self) {
        self.iter()
    }
    */
}

impl<C: 'static + AnyComponent> ComponentHandlerContainer for ComponentContainer<C> {
    fn register_components(&mut self, components: &Components) {
        for component in components.iter_kind::<C>() {
            self.container.push(component)
        }
    }
}

impl<C: AnyComponent> Deref for ComponentContainer<C> {
    type Target = [ComponentRef<C>];

    fn deref(&self) -> &Self::Target {
        &self.container
    }
}

//

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

