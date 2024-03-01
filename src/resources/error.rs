use thiserror::Error;
use miette::Diagnostic;

#[non_exhaustive]
#[derive(Error, Diagnostic, Debug)]
pub enum AssetError {
    #[error("group not found for asset: {0:?}")]
    GroupNotFound(&'static str),

    #[error("asset not found: {0:?}")]
    AssetNotFound(String),
}
