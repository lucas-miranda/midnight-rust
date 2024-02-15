mod asset;
pub use asset::Asset;

mod loader;
pub use loader::*;

mod resource_group;
pub use resource_group::*;

use std::{
    any::TypeId,
    collections::HashMap,
    rc::Weak,
};

#[derive(Default)]
pub struct AssetResources {
    groups: HashMap<TypeId, Box<dyn UnknownAssetResourceGroup>>
}

impl AssetResources {
    pub fn register_loader<T: 'static + Asset>(&mut self) {
        self.groups.insert(
            TypeId::of::<T>(),
            Box::new(AssetResourceGroup::<T>::new(
                AssetLoader::default()
            )),
        );
    }

    /*
    pub fn register_custom_loader<T, L: 'static + ResourceLoader<T>>(&mut self, loader: L) {
        self.groups.insert(TypeId::of::<T>(), Box::new(ResourceGroup::<T>::new(loader)));
    }
    */

    pub fn get_group<T: 'static + Asset>(&self) -> Option<&AssetResourceGroup<T>> {
        self.groups
            .get(&TypeId::of::<T>())
            .and_then(|u| u.as_any().downcast_ref())
    }

    pub fn get_mut_group<T>(&mut self) -> Option<&mut AssetResourceGroup<T>> where
        T: 'static + Asset
    {
        self.groups
            .get_mut(&TypeId::of::<T>())
            .and_then(|u| u.as_any_mut().downcast_mut())
    }

    pub fn get<T: 'static + Asset, S: AsRef<str>>(&self, id: S) -> Option<Weak<T>> {
        self.get_group()
            .and_then(|g| g.get(id))
    }
}
