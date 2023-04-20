mod entities;
pub use entities::Entities;

mod entities_iter;
pub use entities_iter::EntitiesIter;

mod entity_builder;
pub use entity_builder::EntityBuilder;

use crate::ecs::component::{
    AnyComponent,
    Components
};

pub type EntityId = u32;

pub struct Entity {
    id: EntityId,
    components: Components,
}

impl Entity {
    pub(super) fn new(id: EntityId) -> Self {
        Self {
            id,
            components: Components::new(id),
        }
    }

    pub fn id(&self) -> EntityId {
        self.id
    }

    /*
    pub fn create_component<C>(&mut self)  -> Option<C> where
        C: AnyComponent + Default + 'static
    {
        self.register_component(C::default())
    }
    */

    pub fn register_component<C>(&mut self, component: C) -> Option<C> where
        C: AnyComponent + 'static
    {
        self.components.internal_register(component, self.id())
    }

    pub fn components(&self) -> &Components {
        &self.components
    }
}
