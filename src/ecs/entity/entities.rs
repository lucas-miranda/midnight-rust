use std::{
    cell::{Ref, RefCell, RefMut},
    collections::hash_map::HashMap,
    rc::Rc,
};
use super::{Entity, EntityBuilder, EntityId, EntitiesIter};

pub struct Entities {
    entries: HashMap<EntityId, Rc<RefCell<Entity>>>,
    next_id: EntityId,
    pub(super) setup_entity: Option<Box<dyn FnMut(RefMut<Entity>)>>,
}

impl Entities {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            next_id: EntityId::default(),
            setup_entity: None,
        }
    }

    pub fn with_setup<F: 'static + FnMut(RefMut<Entity>)>(mut self, setup_entity: F) -> Self {
        self.setup_entity = Some(Box::new(setup_entity));
        self
    }

    pub fn create(&mut self) -> EntityBuilder {
        let id = self.next_id();
        EntityBuilder::new(Entity::new(id), self)
    }

    pub fn get(&self, id: EntityId) -> Option<Ref<Entity>> {
        self.entries
            .get(&id)
            .map(|e| e.borrow())
    }

    pub fn get_mut(&mut self, id: EntityId) -> Option<RefMut<Entity>> {
        self.entries
            .get_mut(&id)
            .map(|e| e.borrow_mut())
    }

    pub fn count(&self) -> usize {
        self.entries.len()
    }

    pub fn iter(&self) -> EntitiesIter {
        self.entries.values().into()
    }

    /*
    pub fn iter<'a, F>(&'a self) -> EntitiesIter<'a, F> where
        F: FnMut(Rc<RefCell<Entity>>) -> Ref<'a, Entity>
    {
        self.entries
            .values()
            .map(|e| e.borrow())
            .into()
    }
    */

    pub(super) fn register(&mut self, entity: Rc<RefCell<Entity>>) {
        let id = entity.borrow().id();

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
