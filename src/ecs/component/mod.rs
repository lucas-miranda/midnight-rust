mod any_component;
pub use any_component::AnyComponent;

mod component_entry;
pub use component_entry::{ComponentEntry, RawComponentEntry};

mod components;
pub use components::Components;

mod containers;
pub use containers::*;

mod queries;
pub use queries::*;

mod refs;
pub use refs::*;

//

use bitflags::bitflags;

bitflags! {
    #[derive(Eq, PartialEq, Hash, Clone)]
    pub struct ComponentAttribute: u32 {
        const None   = 0b00000000;
        const Unique = 0b00000001;
    }
}

/// A component which can be registered to an entity.
pub trait Component : 'static {
    fn attributes(&self) -> ComponentAttribute;

    /// Registered to an Entity.
    fn registered(&mut self, components: &mut Components);

    /// Unregistered from an Entity.
    fn unregistered(&mut self);
}

#[derive(Default)]
pub struct EmptyComponent {
}

impl Component for EmptyComponent {
    fn attributes(&self) -> ComponentAttribute {
        ComponentAttribute::None
    }

    fn registered(&mut self, _components: &mut Components) {
    }

    fn unregistered(&mut self) {
    }
}
