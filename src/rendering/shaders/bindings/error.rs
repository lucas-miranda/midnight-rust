use thiserror::Error;
use miette::Diagnostic;

use super::BindingsDescriptorEntry;

#[derive(Error, Diagnostic, Debug)]
pub enum BindingsError {
    #[error("expecting a value to binding({at_index}) '{:?}'", expecting)]
    EmptyValue { expecting: BindingsDescriptorEntry, at_index: usize },

    #[error("a binding with type '{:?}' was not found", expecting)]
    NotFound { expecting: BindingsDescriptorEntry },
}
