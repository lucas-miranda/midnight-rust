use std::{
    any::Any,
    collections::HashMap,
    path::Path,
    rc::{Rc, Weak},
};

use super::{Asset, AssetResourceLoader};

pub struct AssetResourceGroup<T: Asset> {
    loader: Box<dyn AssetResourceLoader<T>>,
    entries: HashMap<String, Rc<T>>,
}

impl<T: Asset> AssetResourceGroup<T> {
    pub fn new<L: 'static + AssetResourceLoader<T>>(loader: L) -> Self {
        Self {
            loader: Box::new(loader),
            entries: HashMap::default(),
        }
    }

    pub fn get<S: AsRef<str>>(&self, id: S) -> Option<Weak<T>> {
        self.entries.get(id.as_ref()).map(|r| Rc::downgrade(r))
    }

    /*
    pub fn get_mut<S: AsRef<String>>(&mut self, id: S) -> Option<Weak<&mut T>> {
        self.entries.get_mut(id.as_ref()).map(|r| Rc::downgrade(r))
    }
    */

    pub fn register<S: Into<String>>(&mut self, id: S, resource: T) {
        self.entries.insert(id.into(), Rc::new(resource));
    }

    pub fn load<I, P>(&mut self, id: I, path: P) -> Result<(), T::E> where
        I: Into<String>,
        P: AsRef<Path>,
    {
        let res = self.loader.load(path.as_ref())?;
        self.register(id.into(), res);

        Ok(())
    }
}

pub trait UnknownAssetResourceGroup {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: 'static + Asset> UnknownAssetResourceGroup for AssetResourceGroup<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
