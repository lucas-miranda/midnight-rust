use std::ops::Deref;

use crate::ecs::component::{
    ComponentStrongRef,
    AnyComponent,
};

use super::ComponentQuery;

pub struct ExplicitComponentQuery<'a, C: AnyComponent> {
    components: Vec<ComponentStrongRef<'a, C>>,
}

impl<'a, C: AnyComponent> ExplicitComponentQuery<'a, C> {
    pub(in crate::ecs::component) fn new(components: Vec<ComponentStrongRef<'a, C>>) -> Self {

        Self {
            components,
        }
    }

    /// Retrieve first component found
    pub fn component(&self) -> Result<&ComponentStrongRef<C>, &'static str> {
        if let Some(element) = self.components.first() {
            return Ok(element)
        }

        Err("Not found")
    }
}

impl<'a, C: AnyComponent> ComponentQuery for ExplicitComponentQuery<'a, C> {
    fn count(&self) -> usize {
        self.components.len()
    }
}

impl<'a, C: AnyComponent> Deref for ExplicitComponentQuery<'a, C> {
    type Target = Vec<ComponentStrongRef<'a, C>>;

    fn deref(&self) -> &Self::Target {
        &self.components
    }
}
