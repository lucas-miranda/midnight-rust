mod component_ref;
pub use component_ref::ComponentRef;

mod components;
pub use components::Components;

mod containers;
pub use containers::*;

/// A component which can be registered to an entity.
pub trait Component {
    fn as_unique(&self) -> Option<&dyn ComponentUnique>;
}

/// An unique component which can be registered to an entity.
/// The unique property will be enforced, so only one of each implementor can be registered.
pub trait ComponentUnique : Component {
}

#[derive(Default)]
pub struct EmptyComponent {
}

impl Component for EmptyComponent {
    fn as_unique(&self) -> Option<&dyn ComponentUnique> {
        None
    }
}
