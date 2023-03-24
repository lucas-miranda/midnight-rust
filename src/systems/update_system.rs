use std::{
    cell::RefCell,
    rc::Rc,
};

use crate::ecs::{
    component::{
        AnyComponent,
        ComponentFnContainer,
        ComponentHandlerContainer,
        ComponentQuery,
        EmptyComponent,
    },
    system::System,
};

#[derive(Default)]
pub struct UpdateSystem {
}

impl UpdateSystem {
    fn component_filter(_component: Rc<RefCell<dyn AnyComponent>>) -> bool {
        //component.borrow().as_updatable().is_some()
        true
    }
}

impl System for UpdateSystem {
    type Component = EmptyComponent;
    type Container = ComponentFnContainer;

    fn setup(&mut self) {
    }

    fn run(&mut self, query: <Self::Container as ComponentHandlerContainer>::Query) {
        println!("[UpdateSystem] {} captured components", query.count());

        for component_ref in query.iter() {
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
