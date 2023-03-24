use std::ops::Deref;

use crate::ecs::component::{
    AnyComponent,
    Components,
    ComponentRef,
    ExplicitComponentQuery,
};

use super::ComponentHandlerContainer;

#[derive(Default)]
pub struct ComponentContainer<'a, C: 'static + AnyComponent> {
    container: Vec<ComponentRef<C>>,
    phantom: std::marker::PhantomData<&'a ()>
}

impl<'a, C: 'static + AnyComponent> ComponentContainer<'a, C> {
    /*
    /// Retrieve first component found
    pub fn component<'a>(&'a self) -> Result<&ComponentStrongRef<C>, &'static str> {
        if let Some(element) = self.container.first() {
            return Ok(element)
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
    */

    pub fn register(&mut self, component: ComponentRef<C>) {
        self.container.push(component);
    }

    pub fn count(&self) -> usize {
        self.container.len()
    }
}

impl<'a, C: 'static + AnyComponent> ComponentHandlerContainer for ComponentContainer<'a, C> {
    type Query = ExplicitComponentQuery<'a, C>;

    fn capture_components(&mut self, components: &Components) {
        for component in components.iter_kind::<C>() {
            self.container.push(component)
        }
    }

    fn query(self) -> Self::Query {
        ExplicitComponentQuery::new(
            self.container
                .into_iter()
                .map(|a| a.consume().unwrap())
                .collect()
        )
    }
}

impl<'a, C: AnyComponent> Deref for ComponentContainer<'a, C> {
    type Target = [ComponentRef<C>];

    fn deref(&self) -> &Self::Target {
        &self.container
    }
}
