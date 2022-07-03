use std::{
    cell::RefCell,
    rc::Rc,
};

use crate::ecs::{
    component::{AnyComponent, ComponentFnContainer},
    System,
};

#[derive(Default)]
pub struct UpdateSystem {
}

impl UpdateSystem {
    fn component_filter(component: Rc<RefCell<dyn AnyComponent>>) -> bool {
        component.borrow().as_updatable().is_some()
    }
}

impl System for UpdateSystem {
    type Container = ComponentFnContainer;

    fn setup(&mut self) {
    }

    fn run(&mut self, container: &mut Self::Container) {
        println!("[UpdateSystem] {} captured components", container.count());

        for component_ref in container.iter() {
            if let Ok(strong_ref) = component_ref.retrieve() {
                let mut component = strong_ref.borrow_mut();
                component.as_updatable_mut()
                    .unwrap()
                    .update();
            }
        }
    }

    fn create_container(&self) -> Self::Container {
        Self::Container::new(Self::component_filter)
    }
}
