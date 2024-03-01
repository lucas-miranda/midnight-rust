pub mod component;

mod domain;
use std::collections::HashMap;

pub use domain::Domain;

pub mod entity;
pub mod system;

use crate::{time::DeltaTime, base::ApplicationState};

use self::{system::{System, SystemInterface}, entity::Entities};

pub struct FrameState<'a> {
    pub delta: DeltaTime,
    pub app: &'a mut ApplicationState,
}


#[derive(PartialEq, Eq, Hash)]
pub enum SchedulerStep {
    Update,
    Render,
}

pub struct SystemScheduler {
    systems: HashMap<SchedulerStep, Vec<system::SystemInterface>>,
}

impl SystemScheduler {
    pub fn new() -> Self {
        let mut systems = HashMap::default();
        systems.insert(SchedulerStep::Update, Vec::default());
        systems.insert(SchedulerStep::Render, Vec::default());

        Self {
            systems,
        }
    }

    pub fn register<S: 'static + System>(&mut self, step: SchedulerStep, sys: S) {
        let s = self.get_mut_step(&step);
        s.push(SystemInterface::wrap(sys));
    }

    pub fn run(&mut self, step: &SchedulerStep, entities: &Entities, state: &mut FrameState) {
        for sys in self.get_mut_step(step) {
            sys.run(entities.iter(), state)
        }
    }

    fn get_step(&self, step: &SchedulerStep) -> &Vec<system::SystemInterface> {
        self.systems.get(step).unwrap()
    }

    fn get_mut_step(&mut self, step: &SchedulerStep) -> &mut Vec<system::SystemInterface> {
        self.systems.get_mut(step).unwrap()
    }
}
