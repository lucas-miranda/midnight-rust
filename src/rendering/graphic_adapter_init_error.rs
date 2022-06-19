use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};

use super::backend::RenderBackendBuildError;

#[derive(Debug)]
pub enum GraphicAdapterInitError {
    BackendFailed(RenderBackendBuildError),
}

impl Error for GraphicAdapterInitError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self {
            Self::BackendFailed(back_err) => Some(back_err),
        }
    }
}

impl Display for GraphicAdapterInitError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            Self::BackendFailed(back_err)
                => write!(f, "Backend failed to initialize: {}", back_err),
        }
    }
}
