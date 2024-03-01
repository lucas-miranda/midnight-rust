mod asset;
pub use asset::*;

mod error;
pub use error::AssetError;

mod resource_group;
pub use resource_group::*;

use std::{
    any::{TypeId, type_name},
    cell::{Ref, RefMut},
    collections::HashMap,
};

#[derive(Default)]
pub struct AssetResources {
    groups: HashMap<TypeId, Box<dyn UnknownAssetResourceGroup>>
}

impl AssetResources {
    pub fn register_loader<T: 'static>(&mut self) {
        self.groups.insert(
            TypeId::of::<T>(),
            Box::new(AssetResourceGroup::<T>::new()),
        );
    }

    pub fn get_group<T: 'static>(&self) -> Result<&AssetResourceGroup<T>, AssetError> {
        self.groups
            .get(&TypeId::of::<T>())
            .and_then(|u| u.as_any().downcast_ref())
            .ok_or_else(|| AssetError::GroupNotFound(type_name::<T>()))
    }

    pub fn get_mut_group<T>(&mut self) -> Result<&mut AssetResourceGroup<T>, AssetError> where
        T: 'static
    {
        self.groups
            .get_mut(&TypeId::of::<T>())
            .and_then(|u| u.as_any_mut().downcast_mut())
            .ok_or_else(|| AssetError::GroupNotFound(type_name::<T>()))
    }

    pub fn get_asset<T: 'static, S: AsRef<str>>(&self, id: S) -> Result<&Asset<T>, AssetError> {
        self.get_group()
            .and_then(|g| g.get_asset(id))
    }

    pub fn get<T: 'static, S: AsRef<str>>(&self, id: S) -> Result<Ref<'_, T>, AssetError> {
        self.get_group()
            .and_then(|g| g.get(id))
    }

    pub fn get_mut<T: 'static, S: AsRef<str>>(&mut self, id: S) -> Result<RefMut<'_, T>, AssetError> {
        self.get_mut_group()
            .and_then(|g| g.get_mut(id))
    }
}
