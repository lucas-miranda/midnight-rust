use std::collections::hash_map::HashMap;
use super::{Entity, EntityBuilder, EntityId, EntitiesIter};

pub struct Entities {
    entries: HashMap<EntityId, Entity>,
    next_id: EntityId,
    pub(super) setup_entity: Option<Box<dyn FnMut(&mut Entity)>>,
}

impl Entities {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            next_id: EntityId::default(),
            setup_entity: None,
        }
    }

    pub fn with_setup<F: 'static + FnMut(&mut Entity)>(mut self, setup_entity: F) -> Self {
        self.setup_entity = Some(Box::new(setup_entity));
        self
    }

    pub fn create(&mut self) -> EntityBuilder {
        let id = self.next_id();
        EntityBuilder::new(Entity::new(id), self)
    }

    pub fn get(&self, id: EntityId) -> Option<&Entity> {
        self.entries.get(&id)
    }

    pub fn get_mut(&mut self, id: EntityId) -> Option<&mut Entity> {
        self.entries.get_mut(&id)
    }

    pub fn count(&self) -> usize {
        self.entries.len()
    }

    pub fn iter(&self) -> EntitiesIter {
        self.entries.values().into()
    }

    pub(super) fn register(&mut self, entity: Entity) {
        let id = entity.id();

        assert!(
            self.entries.insert(id, entity).is_none(),
            "Something very wrong happened when registering an entity with id {}.",
            id,
        );
    }

    fn next_id(&mut self) -> EntityId {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}

impl Default for Entities {
    fn default() -> Self {
        Self::new()
    }
}
