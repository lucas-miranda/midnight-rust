use std::any::Any;
use crate::ecs::{
    component::ComponentQuery,
    entity::EntitiesIter,
};
use crate::input;

use super::System;

pub struct SystemInterface {
    system: Box<dyn Any>,
    setup_fn: Box<dyn FnMut(&mut Box<dyn Any>)>,
    input_fn: Box<dyn FnMut(&mut Box<dyn Any>, EntitiesIter<'_>, &input::DeviceEvent)>,
    run_fn: Box<dyn FnMut(&mut Box<dyn Any>, EntitiesIter<'_>)>,
}

impl SystemInterface {
    pub fn wrap<'a, Q: ComponentQuery + 'static, S: System<Query<'a> = Q> + 'static>(system: S) -> Self {
        SystemInterface {
            system: Box::new(system),
            setup_fn: Box::new(|boxed_system| {
                let sys = boxed_system.downcast_mut::<S>().unwrap();
                sys.setup();
            }),
            input_fn: Box::new(|boxed_system, entities, event| {
                let sys = boxed_system.downcast_mut::<S>().unwrap();

                let mut query = sys.create_query();

                for entity in entities {
                    query.capture_components(entity.borrow().components());
                }

                sys.input(query, event)
            }),
            run_fn: Box::new(|boxed_system, entities| {
                let sys = boxed_system.downcast_mut::<S>().unwrap();
                let mut query = sys.create_query();

                for entity in entities {
                    query.capture_components(entity.borrow().components());
                }

                sys.run(query);
            }),
        }
    }

    pub fn setup(&mut self) {
        (*self.setup_fn)(&mut self.system)
    }

    pub fn input(&mut self, entities: EntitiesIter<'_>, event: &input::DeviceEvent) {
        (*self.input_fn)(&mut self.system, entities, event)
    }

    pub fn run(&mut self, entities: EntitiesIter<'_>) {
        (*self.run_fn)(&mut self.system, entities)
    }
}
