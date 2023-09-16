mod bindings;
pub use bindings::*;

mod bindings_descriptor;
pub use bindings_descriptor::*;

mod error;
pub use error::*;

const UNIFORM_BINDING_ALIGNMENT: usize = 16;
