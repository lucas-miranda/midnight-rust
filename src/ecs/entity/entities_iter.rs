use std::{
    cell::RefCell,
    collections::hash_map,
    rc::Rc,
};

use super::{Entity, EntityId};

pub struct EntitiesIter<'a> {
    values: hash_map::Values<'a, EntityId, Rc<RefCell<Entity>>>,
}

impl<'a> From<hash_map::Values<'a, EntityId, Rc<RefCell<Entity>>>> for EntitiesIter<'a> {
    fn from(values: hash_map::Values<'a, EntityId, Rc<RefCell<Entity>>>) -> Self {
        Self {
            values,
        }
    }
}

impl<'a> IntoIterator for EntitiesIter<'a> {
    type Item = &'a Rc<RefCell<Entity>>;
    type IntoIter = hash_map::Values<'a, EntityId, Rc<RefCell<Entity>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.values
    }
}

/*
pub struct EntitiesIter<'a, F> where
    F: FnMut(Rc<RefCell<Entity>>) -> Ref<'a, Entity>
{
    values: std::iter::Map<std::collections::hash_map::Values<'a, EntityId, Rc<RefCell<Entity>>>, F>,
}

impl<'a, F> From<std::iter::Map<hash_map::Values<'a, EntityId, Rc<RefCell<Entity>>>, F>> for EntitiesIter<'a, F> where
    F: FnMut(Rc<RefCell<Entity>>) -> Ref<'a, Entity>
{
    fn from(values: std::iter::Map<hash_map::Values<'a, EntityId, Rc<RefCell<Entity>>>, F>) -> Self {
        Self {
            values,
        }
    }
}

impl<'a, F> IntoIterator for EntitiesIter<'a, F> where
    F: FnMut(Rc<RefCell<Entity>>) -> Ref<'a, Entity>
{
    type Item = &'a Ref<'a, Entity>;
    type IntoIter = hash_map::Values<'a, EntityId, Ref<'a, Entity>>;

    fn into_iter(self) -> Self::IntoIter {
        self.values
    }
}
*/


/*
impl<'a> From<hash_map::Values<'a, EntityId, Ref<'a, Entity>>> for EntitiesIter<'a> {
    fn from(values: hash_map::Values<'a, EntityId, Ref<'a, Entity>>) -> Self {
        Self {
            values,
        }
    }
}
*/
