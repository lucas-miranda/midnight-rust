use std::ops::Deref;
use crate::ecs::component::{
    AnyComponent,
    Components,
    ComponentStrongRef,
};

use super::ComponentHandlerContainer;

//#[derive(Default)]
pub struct ComponentContainer<'a, C: 'static + AnyComponent> {
    container: Vec<ComponentStrongRef<'a, C>>,
}

impl<'a, C: 'static + AnyComponent> Default for ComponentContainer<'a, C> {
    fn default() -> Self {
        Self {
            container: Vec::default(),
        }
    }
}

impl<'a, C: 'static + AnyComponent> ComponentContainer<'a, C> {
    /// Retrieve first component found
    pub fn component(&self) -> Result<&ComponentStrongRef<C>, &'static str> {
        if let Some(element) = self.container.first() {
            return Ok(element)
        }

        Err("Not found")
    }

    pub fn count(&self) -> usize {
        self.container.len()
    }
}

impl<'a, C: 'static + AnyComponent> ComponentHandlerContainer for ComponentContainer<'a, C> {
    type ComponentQuery = C;

    fn capture_components(&mut self, components: &Components) {
        for component in components.iter_kind::<C>() {
            self.container.push(component.consume().unwrap())
        }
    }
}

impl<'a, C: AnyComponent> Deref for ComponentContainer<'a, C> {
    type Target = [ComponentStrongRef<'a, C>];

    fn deref(&self) -> &Self::Target {
        &self.container
    }
}
