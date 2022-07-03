mod component_entry;
pub use component_entry::{ComponentEntry, RawComponentEntry};

mod component_ref;
pub use component_ref::{ComponentRef, ComponentStrongRef, ComponentAnyRef};

mod components;
pub use components::Components;

mod containers;
pub use containers::*;

/// A component which can be registered to an entity.
pub trait Component {
    fn as_unique(&self) -> Option<&dyn ComponentUnique>;
    fn as_unique_mut(&mut self) -> Option<&mut dyn ComponentUnique>;
    fn as_updatable(&self) -> Option<&dyn ComponentUpdatable>;
    fn as_updatable_mut(&mut self) -> Option<&mut dyn ComponentUpdatable>;

    /// Registered to an Entity.
    fn registered(&mut self, components: &mut Components);

    /// Unregistered from an Entity.
    fn unregistered(&mut self);
}

/// An unique component which can be registered to an entity.
/// The unique property will be enforced, so only one of each implementor can be registered.
pub trait ComponentUnique : Component {
}

/// Component supports receive update calls.
pub trait ComponentUpdatable : Component {
    //fn update(&mut self, delta: &crate::time::DeltaTime);
    fn update(&mut self);
}

#[derive(Default)]
pub struct EmptyComponent {
}

impl Component for EmptyComponent {
    fn as_unique(&self) -> Option<&dyn ComponentUnique> {
        None
    }

    fn as_unique_mut(&mut self) -> Option<&mut dyn ComponentUnique> {
        None
    }

    fn as_updatable(&self) -> Option<&dyn ComponentUpdatable> {
        None
    }

    fn as_updatable_mut(&mut self) -> Option<&mut dyn ComponentUpdatable> {
        None
    }

    fn registered(&mut self, components: &mut Components) {
    }

    fn unregistered(&mut self) {
    }
}

impl AnyComponent for EmptyComponent {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn as_component(&self) -> &dyn Component {
        self
    }

    fn as_component_mut(&mut self) -> &mut dyn Component {
        self
    }
}

use std::any::Any;

pub trait AnyComponent: Any + Component {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn as_component(&self) -> &dyn Component;
    fn as_component_mut(&mut self) -> &mut dyn Component;
}
