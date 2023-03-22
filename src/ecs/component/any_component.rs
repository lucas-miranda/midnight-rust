use std::any::Any;
use super::Component;

pub trait AnyComponent: Any + Component {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn as_component(&self) -> &dyn Component;
    fn as_component_mut(&mut self) -> &mut dyn Component;
}

impl<T> AnyComponent for T where
    T: Any + Component
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn as_component(&self) -> &dyn Component {
        self
    }

    fn as_component_mut(&mut self) -> &mut dyn Component {
        self
    }
}
