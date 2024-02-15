use std::path::Path;

pub trait Asset {
    type E: std::error::Error;

    fn load(path: &Path) -> Result<Self, Self::E> where Self: Sized;
}
