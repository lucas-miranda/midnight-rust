use std::rc::Weak;

use crate::{
    ecs::component::{
        Component,
        Components,
        ComponentAttribute,
    },
    math::Vector2,
};

macro_rules! retrieve_parent {
    ($self:expr, $identifier:ident) => {
        match &$self.parent {
            Some(p) => p.upgrade().unwrap().$identifier(),
            None => Default::default(),
        }
    };
}

pub type Position = Vector2<f32>;
pub type Rotation = f32;
pub type Scale = Vector2<f32>;

#[derive(Default, Debug)]
pub struct Transform {
    pub local_position: Position,
    pub local_rotation: Rotation,
    pub local_scale: Scale,
    parent: Option<Weak<Transform>>,
}

impl Transform {
    pub fn position(&self) -> Position {
        self.local_position + retrieve_parent!(self, position)
    }

    pub fn rotation(&self) -> Rotation {
        self.local_rotation + retrieve_parent!(self, rotation)
    }

    pub fn scale(&self) -> Scale {
        self.local_scale + retrieve_parent!(self, scale)
    }
}

impl Component for Transform {
    fn attributes(&self) -> ComponentAttribute {
        ComponentAttribute::Unique
    }

    fn registered(&mut self, _components: &mut Components) {
    }

    fn unregistered(&mut self) {
    }
}
