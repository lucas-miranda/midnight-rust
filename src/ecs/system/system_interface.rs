use std::any::Any;
use crate::ecs::{
    component::ComponentHandlerContainer,
    entity::{EntityId, EntitiesIter},
};

use super::System;

pub struct SystemInterface {
    system: Box<dyn Any>,
    setup_fn: Box<dyn FnMut(&mut Box<dyn Any>)>,
    run_fn: Box<dyn FnMut(&mut Box<dyn Any>, EntitiesIter<'_>)>,
    //run_fn: Box<dyn FnMut(&mut Box<dyn Any>, Box<&mut dyn Any>)>,
    //create_container_fn: Box<dyn FnMut(&mut Box<dyn Any>, &[Entity]) -> Box<dyn Any>>,
}

impl SystemInterface {
    pub fn wrap<C: ComponentHandlerContainer + 'static, S: System<Container = C> + 'static>(system: S) -> Self {
        SystemInterface {
            system: Box::new(system),
            setup_fn: Box::new(|boxed_system| {
                let sys = boxed_system.downcast_mut::<S>().unwrap();
                sys.setup();
            }),
            run_fn: Box::new(|boxed_system, entities| {
                let sys = boxed_system.downcast_mut::<S>().unwrap();
                let mut container = sys.create_container();

                for entity in entities {
                    //let e: &Entity = entity;
                    //println!("[{}]", entity.id());
                    container.register_components(entity.borrow().components());
                    /*
                    entity.components()
                        .iter()
                        .for_each(|c| {
                            container.try_register(c);
                        })
                    */
                }

                // TODO  register entities' components into container

                sys.run(&mut container);

                /*
                if !container.is::<C>() {
                    panic!("Container type mismatch.")
                }

                let sys = boxed_system.downcast_mut::<S>().unwrap();
                sys.run((*container).downcast_mut().unwrap()); // TODO  don't unwrap here, check first if container is valid
                */
            }),
            /*
            create_container_fn: Box::new(|boxed_system, entities| {
                Box::new(C::default())
            })
            */
        }
    }

    pub fn setup(&mut self) {
        (*self.setup_fn)(&mut self.system)
    }

    pub fn run(&mut self, entities: EntitiesIter<'_>) {
        (*self.run_fn)(&mut self.system, entities)
    }

    /*
    pub fn run<'a, C: ComponentHandlerContainer + 'static>(&mut self, container: &'a mut C) {
        (*self.run_fn)(&mut self.system, Box::new(container))
    }
    */

    /*
    pub fn create_container(&mut self, entities: &[Entity]) -> impl ComponentHandlerContainer {
        (*self.create_container_fn)(&mut self.system, entities).downcast::<dyn ComponentHandlerContainer>()
    }
    */
}
