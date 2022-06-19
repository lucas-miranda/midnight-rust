use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};

#[derive(Debug)]
pub enum GraphicBackendInitError {
    //HALInitFailed(super::HALInitError),
}

/*
impl Error for GraphicBackendInitError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self {
            Self::HALInitFailed(hal_err) => Some(hal_err),
        }
    }
}

impl Display for GraphicBackendInitError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            Self::HALInitFailed(hal_err) => write!(f, "Failed to initialize hardware abstraction layer: {}", hal_err),
        }
    }
}
*/
