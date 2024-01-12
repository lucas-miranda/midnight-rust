use std::any::Any;

use crate::base::ApplicationState;
use crate::ecs::{
    component::BaseQuery,
    entity::EntitiesIter,
    FrameState,
};

use super::System;

pub struct SystemInterface {
    system: Box<dyn Any>,
    setup_fn: Box<dyn FnMut(&mut Box<dyn Any>)>,
    input_fn: Box<dyn FnMut(&mut Box<dyn Any>, EntitiesIter<'_>, &mut ApplicationState)>,
    run_fn: Box<dyn FnMut(&mut Box<dyn Any>, EntitiesIter<'_>, &mut FrameState<'_>)>,
}

impl SystemInterface {
    pub fn wrap<'a, Q: BaseQuery + 'static, S: System<Query<'a> = Q> + 'static>(system: S) -> Self {
        SystemInterface {
            system: Box::new(system),
            setup_fn: Box::new(|boxed_system| {
                let sys = boxed_system.downcast_mut::<S>().unwrap();
                sys.setup();
            }),
            input_fn: Box::new(|boxed_system, entities, app_state| {
                let sys = boxed_system.downcast_mut::<S>().unwrap();

                let mut query = sys.create_query();

                for entity in entities {
                    query.capture_components(entity.borrow().components());
                }

                sys.input(query, app_state)
            }),
            run_fn: Box::new(|boxed_system, entities, state| {
                let sys = boxed_system.downcast_mut::<S>().unwrap();
                let mut query = sys.create_query();

                for entity in entities {
                    query.capture_components(entity.borrow().components());
                }

                sys.run(query, state);
            }),
        }
    }

    pub fn setup(&mut self) {
        (*self.setup_fn)(&mut self.system)
    }

    pub fn input(&mut self, entities: EntitiesIter<'_>, app_state: &mut ApplicationState) {
        (*self.input_fn)(&mut self.system, entities, app_state)
    }

    pub fn run(&mut self, entities: EntitiesIter<'_>, state: &mut FrameState) {
        (*self.run_fn)(&mut self.system, entities, state)
    }
}
