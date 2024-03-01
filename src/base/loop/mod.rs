mod continuous;

pub use continuous::ContinuousLoop;

use std::error::Error;
use crate::{window::WindowContext, ecs::{SystemScheduler, entity::Entities}};
use super::{ApplicationError, ApplicationState};

pub type InitFn = Box<dyn for<'a> FnOnce(&'a mut ApplicationState, InitState<'a>) -> Result<(), Box<dyn Error + 'static>>>;

pub trait ApplicationLoop {
    fn new(window_context: WindowContext) -> Self;
    fn run(self, init_fn: Option<InitFn>) -> Result<(), ApplicationError>;
}

pub struct InitState<'a> {
    pub window_context: &'a mut WindowContext,
    pub system_scheduler: &'a mut SystemScheduler,
    pub entities: &'a mut Entities,
}
