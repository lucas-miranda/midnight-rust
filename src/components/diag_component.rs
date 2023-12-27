use crate::ecs::component::{
    Component,
    ComponentAttribute,
    Components,
};

#[derive(Default)]
pub struct DiagComponent {
    pub fps: u32,
}

impl Component for DiagComponent {
    fn attributes(&self) -> ComponentAttribute {
        ComponentAttribute::Unique
    }

    fn registered(&mut self, _components: &mut Components) {
    }

    fn unregistered(&mut self) {
    }
}
