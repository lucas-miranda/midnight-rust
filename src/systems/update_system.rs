use crate::ecs::{
    component::{
        ComponentFnContainer,
        ComponentStrongAnyRef,
    },
    system::System,
};
use crate::input;

#[derive(Default)]
pub struct UpdateSystem {
}

impl UpdateSystem {
    fn component_filter(_component: &ComponentStrongAnyRef) -> bool {
        //component.borrow().as_updatable().is_some()
        true
    }
}

impl System for UpdateSystem {
    type Container = ComponentFnContainer;

    fn setup(&mut self) {
    }

    fn input(&mut self, _container: Self::Container, _event: &input::DeviceEvent) {
    }

    fn run(&mut self, container: Self::Container) {
        //println!("[UpdateSystem] {} captured components", query.count());

        for component_ref in container.iter() {
            let mut component = component_ref.borrow_mut();

            //let updateStep: Option<&dyn UpdateStep> = component.as_any().downcast_ref();



            //component.run();

            /*
            component.as_updatable_mut()
                .unwrap()
                .update();
            */
        }
    }

    fn create_container(&self) -> Self::Container {
        Self::Container::new(Self::component_filter)
        //Self::Container::default()
    }
}
