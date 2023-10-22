mod application;
pub use application::Application;

mod application_error;
pub use application_error::ApplicationError;

mod r#loop;
pub use r#loop::*;

use std::{rc::Rc, cell::RefCell};
use crate::{
    input::Input,
    rendering::GraphicAdapter,
    time::Time,
    window::Window,
};

pub struct ApplicationState {
    pub main_window: Window,
    pub time: Time,
    // TODO  is using Rc<RefCell<>> here a good solution?
    pub graphic_adapter: Rc<RefCell<GraphicAdapter>>,
    pub input: Input,
}

/*
pub trait WaitLoop {
}
*/
