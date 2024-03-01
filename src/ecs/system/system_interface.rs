use std::any::Any;

use crate::ecs::{
    component::BaseQuery,
    entity::EntitiesIter,
    FrameState,
};

use super::System;

pub(crate) struct SystemInterface {
    system: Box<dyn Any>,
    run_fn: Box<dyn FnMut(&mut Box<dyn Any>, EntitiesIter<'_>, &mut FrameState<'_>)>,
}

impl SystemInterface {
    pub fn wrap<'a, Q: BaseQuery + 'static, S: System<Query<'a> = Q> + 'static>(system: S) -> Self {
        SystemInterface {
            system: Box::new(system),
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

    pub fn run(&mut self, entities: EntitiesIter<'_>, state: &mut FrameState) {
        (*self.run_fn)(&mut self.system, entities, state)
    }
}
