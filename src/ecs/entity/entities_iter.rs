use std::collections::hash_map;
use super::{Entity, EntityId};

pub struct EntitiesIter<'a> {
    values: hash_map::Values<'a, EntityId, Entity>,
}

impl<'a> EntitiesIter<'a> {
}

impl<'a> From<hash_map::Values<'a, EntityId, Entity>> for EntitiesIter<'a> {
    fn from(values: hash_map::Values<'a, EntityId, Entity>) -> Self {
        Self {
            values,
        }
    }
}

impl<'a> IntoIterator for EntitiesIter<'a> {
    type Item = &'a Entity;
    type IntoIter = hash_map::Values<'a, EntityId, Entity>;

    fn into_iter(self) -> Self::IntoIter {
        self.values
    }
}
