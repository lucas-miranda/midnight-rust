use std::{
    any::Any,
    cell::{Ref, RefMut},
    collections::HashMap,
    path::Path,
};

use super::{Asset, AssetPathLoad, AssetError};

pub struct AssetResourceGroup<T> {
    entries: HashMap<String, Asset<T>>,
}

impl<T> AssetResourceGroup<T> {
    pub fn new() -> Self {
        Self {
            entries: HashMap::default(),
        }
    }

    pub fn get_asset<S: AsRef<str>>(&self, id: S) -> Result<&Asset<T>, AssetError> {
        self.entries
            .get(id.as_ref())
            .ok_or_else(|| AssetError::AssetNotFound(id.as_ref().to_owned()))
    }

    pub fn get<S: AsRef<str>>(&self, id: S) -> Result<Ref<'_, T>, AssetError> {
        self.entries
            .get(id.as_ref())
            .map(|asset| asset.get())
            .ok_or_else(|| AssetError::AssetNotFound(id.as_ref().to_owned()))
    }

    pub fn get_mut<S: AsRef<str>>(&mut self, id: S) -> Result<RefMut<'_, T>, AssetError> {
        self.entries
            .get_mut(id.as_ref())
            .map(|asset| asset.get_mut())
            .ok_or_else(|| AssetError::AssetNotFound(id.as_ref().to_owned()))
    }

    pub fn register_asset<S: Into<String>, A: Into<Asset<T>>>(&mut self, id: S, asset: A) {
        self.entries.insert(id.into(), asset.into());
    }

    pub fn register<S: Into<String>>(&mut self, id: S, resource: T) {
        self.register_asset(id, Asset::new(resource))
    }
}

impl<T: AssetPathLoad> AssetResourceGroup<T> {
    pub fn load<I, P>(&mut self, id: I, path: P) -> Result<(), T::LoadError> where
        I: Into<String>,
        P: AsRef<Path>,
    {
        let res = T::load(path.as_ref())?;
        self.register(id.into(), res);

        Ok(())
    }
}


pub trait UnknownAssetResourceGroup {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: 'static> UnknownAssetResourceGroup for AssetResourceGroup<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
