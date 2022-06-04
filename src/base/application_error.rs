pub use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[non_exhaustive]
#[derive(Debug)]
pub enum ApplicationError {
}

impl Error for ApplicationError {
}

impl Display for ApplicationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}
