use std::ops::Deref;
use crate::ecs::component::{
    AnyComponent,
    Components,
    ComponentQueryIterator,
    ComponentRef,
    ComponentStrongRef,
    ComponentValueRef,
    ComponentValueMutRef,
};

use super::ComponentQuery;

//#[derive(Default)]
pub struct UnitQuery<'a, C: 'static + AnyComponent> {
    container: Option<ComponentStrongRef<'a, C>>,
}

impl<'a, C: 'static + AnyComponent> Default for UnitQuery<'a, C> {
    fn default() -> Self {
        Self {
            container: Option::default(),
        }
    }
}

impl<'a, C: 'static + AnyComponent> UnitQuery<'a, C> {
    pub fn with(component: &'a ComponentRef<C>) -> Self {
        let container = match component.retrieve() {
            Ok(c) => Some(c),
            Err(_) => None,
        };

        Self {
            container
        }
    }

    /// Retrieve first component found
    pub fn component<'r>(&'r self) -> Result<ComponentValueRef<'r, C>, &'static str> {
        if let Some(element) = &self.container {
            return Ok(element.borrow())
        }

        Err("Not found")
    }

    pub fn component_mut<'r>(&'r self) -> Result<ComponentValueMutRef<'r, C>, &'static str> {
        if let Some(element) = &self.container {
            return Ok(element.borrow_mut())
        }

        Err("Not found")
    }

    pub fn capture_component(&mut self, component: &'a ComponentRef<C>) {
        match component.retrieve() {
            Ok(c) => self.container.replace(c),
            Err(_) => None,
        };
    }

    pub fn is_empty(&self) -> bool {
        self.container.is_none()
    }
}

impl<'a, C: 'static + AnyComponent> ComponentQuery for UnitQuery<'a, C> {
    type Target<'t> = ComponentValueRef<'t, C> where Self: 't;

    fn capture_components(&mut self, components: &Components) {
        match components.iter_kind::<C>().next() {
            Some(component) => self.container.replace(component.consume().unwrap()),
            None => None,
        };
    }

    fn iter_components<'i>(&'i self) -> ComponentQueryIterator<'i, Self::Target<'i>> {
        ComponentQueryIterator::new(
            self.container
                .iter()
                .map(|c| c.borrow())
        )
    }
}

impl<'a, C: AnyComponent> Deref for UnitQuery<'a, C> {
    type Target = Option<ComponentStrongRef<'a, C>>;

    fn deref(&self) -> &Self::Target {
        &self.container
    }
}
