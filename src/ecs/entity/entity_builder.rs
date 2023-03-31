use std::ops::{Deref, DerefMut};

use super::{Entity, Entities};

pub struct EntityBuilder<'a> {
    entity: Entity,
    entities: &'a mut Entities,
}

impl<'a> EntityBuilder<'a> {
    pub(super) fn new(mut entity: Entity, entities: &'a mut Entities) -> Self {
        match &mut entities.setup_entity {
            Some(ref mut setup_entity) => setup_entity(&mut entity),
            None => ()
        }

        Self {
            entity,
            entities,
        }
    }

    pub fn build(self) {
        self.entities.register(self.entity)
    }
}

impl<'a> Deref for EntityBuilder<'a> {
    type Target = Entity;

    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}

impl<'a> DerefMut for EntityBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
