use crate::ecs::{
    component::{
        self,
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
    type Query = component::FnQuery;

    fn setup(&mut self) {
    }

    fn input(&mut self, _query: Self::Query, _event: &input::DeviceEvent) {
    }

    fn run(&mut self, query: Self::Query) {
        //println!("[UpdateSystem] {} captured components", query.count());

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

    fn create_query(&self) -> Self::Query {
        Self::Query::new(Self::component_filter)
    }
}
