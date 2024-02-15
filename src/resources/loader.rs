use std::path::Path;
use super::Asset;

pub trait AssetResourceLoader<R: Asset> {
    fn load(&mut self, path: &Path) -> Result<R, R::E>;
}

#[derive(Default)]
pub struct AssetLoader {
}

impl<R: Asset> AssetResourceLoader<R> for AssetLoader {
    fn load(&mut self, path: &Path) -> Result<R, R::E> {
        R::load(path)
    }
}

