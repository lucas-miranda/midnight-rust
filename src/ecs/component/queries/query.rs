use std::ops::Deref;
use crate::ecs::component::{
    AnyComponent,
    Components,
    ComponentQueryIterator,
    ComponentStrongRef,
    ComponentValueRef,
    ComponentValueMutRef,
};

use super::ComponentQuery;

pub struct Query<'a, C: 'static + AnyComponent> {
    container: Vec<ComponentStrongRef<'a, C>>,
}

impl<'a, C: 'static + AnyComponent> Default for Query<'a, C> {
    fn default() -> Self {
        Self {
            container: Vec::default(),
        }
    }
}

impl<'a, C: 'static + AnyComponent> Query<'a, C> {
    /// Retrieve first component found
    pub fn component<'r>(&'r self) -> Result<ComponentValueRef<'r, C>, &'static str> {
        if let Some(element) = self.container.first() {
            return Ok(element.borrow())
        }

        Err("Not found")
    }

    pub fn component_mut<'r>(&'r self) -> Result<ComponentValueMutRef<'r, C>, &'static str> {
        if let Some(element) = self.container.first() {
            return Ok(element.borrow_mut())
        }

        Err("Not found")
    }

    pub fn count(&self) -> usize {
        self.container.len()
    }
}

impl<'a, C: 'static + AnyComponent> ComponentQuery for Query<'a, C> {
    type Target<'t> = ComponentValueRef<'t, C> where Self: 't;

    fn capture_components(&mut self, components: &Components) {
        for component in components.iter_kind::<C>() {
            self.container.push(component.consume().unwrap())
        }
    }

    fn iter_components<'i>(&'i self) -> ComponentQueryIterator<'i, Self::Target<'i>> {
        ComponentQueryIterator::new(
            self.container
                .iter()
                .map(|c| c.borrow())
        )
    }
}

impl<'a, C: AnyComponent> Deref for Query<'a, C> {
    type Target = [ComponentStrongRef<'a, C>];

    fn deref(&self) -> &Self::Target {
        &self.container
    }
}
