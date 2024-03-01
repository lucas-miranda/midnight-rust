use std::{
    cell::{RefCell, Ref, RefMut},
    path::Path,
    rc::{Rc, Weak},
};

pub struct Asset<T> {
    res: Rc<RefCell<T>>,
}

impl<T> Asset<T> {
    pub fn new(res: T) -> Self {
        Self {
            res: Rc::new(RefCell::new(res)),
        }
    }

    pub fn get(&self) -> Ref<'_, T> {
        self.res.borrow()
    }

    pub fn get_mut(&mut self) -> RefMut<'_, T> {
        self.res.borrow_mut()
    }

    pub fn weak(&self) -> AssetWeak<T> {
        AssetWeak {
            res: Rc::downgrade(&self.res),
        }
    }
}

impl<T: Clone> Asset<T> {
}

impl<T> From<Rc<RefCell<T>>> for Asset<T> {
    fn from(res: Rc<RefCell<T>>) -> Self {
        Self {
            res,
        }
    }
}


pub struct AssetWeak<T> {
    res: Weak<RefCell<T>>,
}

impl<T> AssetWeak<T> {
    pub fn upgrade(&self) -> Option<Asset<T>> {
        self.res
            .upgrade()
            .map(|r| r.into())
    }
}

impl<T> Clone for AssetWeak<T> {
    fn clone(&self) -> Self {
        Self {
            res: self.res.clone(),
        }
    }
}


pub trait AssetPathLoad {
    type LoadError: std::error::Error;

    fn load(path: &Path) -> Result<Self, Self::LoadError> where Self: Sized;
}
