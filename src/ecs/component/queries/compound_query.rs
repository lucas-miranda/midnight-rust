use std::ops::Deref;
use crate::ecs::component::{
    AnyComponent,
    Components,
    ComponentStrongRef,
};

use super::ComponentQuery;

//#[derive(Default)]
pub struct CompoundQuery<'a, Q: ComponentQuery> {
    container: Vec<Q::Target<'a>>,
}

/*
impl<'a, C: 'static + AnyComponent> Default for Query<'a, C> {
    fn default() -> Self {
        Self {
            container: Vec::default(),
        }
    }
}

impl<'a, C: 'static + AnyComponent> Query<'a, C> {
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

impl<'a, C: 'static + AnyComponent> ComponentQuery for Query<'a, C> {
    type Target = C;

    fn capture_components(&mut self, components: &Components) {
        for component in components.iter_kind::<C>() {
            self.container.push(component.consume().unwrap())
        }
    }
}

impl<'a, C: AnyComponent> Deref for Query<'a, C> {
    type Target = [ComponentStrongRef<'a, C>];

    fn deref(&self) -> &Self::Target {
        &self.container
    }
}
*/
