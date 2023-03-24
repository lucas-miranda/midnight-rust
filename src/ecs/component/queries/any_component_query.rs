use std::ops::Deref;

use crate::ecs::component::{
    ComponentAnyRef,
    ComponentStrongAnyRef,
};

use super::ComponentQuery;

pub struct AnyComponentQuery {
    components: Vec<ComponentStrongAnyRef>,
}

impl AnyComponentQuery {
    pub(in crate::ecs::component) fn new(container_components: Vec<ComponentAnyRef>) -> Self {
        let components = container_components
            .into_iter()
            .map(|a| a.retrieve().unwrap())
            .collect();

        Self {
            components,
        }
    }
}

impl ComponentQuery for AnyComponentQuery {
    fn count(&self) -> usize {
        self.components.len()
    }
}

impl Deref for AnyComponentQuery {
    type Target = [ComponentStrongAnyRef];

    fn deref(&self) -> &Self::Target {
        &self.components
    }
}
