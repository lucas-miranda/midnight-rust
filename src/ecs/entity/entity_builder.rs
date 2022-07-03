use std::{
    cell::RefCell,
    rc::Rc,
};

use super::{Entity, Entities};

pub struct EntityBuilder<'a> {
    entity: Rc<RefCell<Entity>>,
    entities: &'a mut Entities,
}

impl<'a> EntityBuilder<'a> {
    pub fn build(self) {
        self.entities.register(self.entity)
    }

    pub(super) fn new(entity: Rc<RefCell<Entity>>, entities: &'a mut Entities) -> Self {
        match &mut entities.setup_entity {
            Some(ref mut setup_entity) => setup_entity(entity.borrow_mut()),
            None => ()
        }

        Self {
            entity,
            entities,
        }
    }
}

impl<'a> std::ops::Deref for EntityBuilder<'a> {
    type Target = Rc<RefCell<Entity>>;

    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}

impl<'a> std::ops::DerefMut for EntityBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
