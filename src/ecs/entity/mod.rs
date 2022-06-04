mod entities;
pub use entities::Entities;

mod entities_iter;
pub use entities_iter::EntitiesIter;

mod entity_builder;
pub use entity_builder::EntityBuilder;

use crate::ecs::component::{Component, Components};
use std::rc::Rc;

pub type EntityId = u32;

pub struct Entity {
    id: EntityId,
    components: Components,
}

impl Entity {
    pub fn id(&self) -> EntityId {
        self.id
    }

    pub fn create_component<C>(&mut self)  -> Option<Rc<C>> where
        C: Component + Default + 'static
    {
        self.register_component(C::default())
    }

    pub fn register_component<C>(&mut self, component: C) -> Option<Rc<C>> where
        C: Component + 'static
    {
        self.components.register(component)
    }

    pub fn components(&self) -> &Components {
        &self.components
    }

    pub(super) fn new(id: EntityId) -> Self {
        Self {
            id,
            components: Components::new(id),
        }
    }
}
