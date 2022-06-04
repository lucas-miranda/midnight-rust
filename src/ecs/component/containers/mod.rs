use std::ops::Deref;

use crate::ecs::component::{Component, Components,  ComponentRef};

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
pub struct ComponentContainer<C: Component> {
    container: Vec<ComponentRef<C>>,
}

impl<C: 'static + Component> ComponentHandlerContainer for ComponentContainer<C> {
    fn register_components(&mut self, components: &Components) {
        for component in components.iter_kind::<C>() {
            self.container.push(component)
        }
    }
}

impl<C: 'static + Component> ComponentContainer<C> {
    /// Retrieve first component found
    pub fn component<'a>(&'a self) -> Option<impl std::ops::Deref<Target = Option<std::rc::Rc<C>>> + 'a> {
        if let Some(element) = self.container.first() {
            return Some(element.as_deref());
        }

        None
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

impl<C: Component> Deref for ComponentContainer<C> {
    type Target = [ComponentRef<C>];

    fn deref(&self) -> &Self::Target {
        &self.container
    }
}
