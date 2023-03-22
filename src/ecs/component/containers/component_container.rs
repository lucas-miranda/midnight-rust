use std::ops::Deref;

use crate::ecs::component::{
    Components,
    ComponentRef,
    ComponentStrongRef,
    AnyComponent,
};

use super::ComponentHandlerContainer;

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
