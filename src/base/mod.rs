mod application;
pub use application::Application;

mod application_error;
pub use application_error::ApplicationError;

mod r#loop;
pub use r#loop::*;

use std::{
    cell::RefCell,
    rc::Rc,
};
use crate::{
    diag::Diagnostics,
    input::Input,
    rendering::GraphicAdapter,
    time::Time,
    window::Window,
};

pub struct ApplicationState {
    pub main_window: Window,
    pub time: Time,
    pub graphic_adapter: Rc<RefCell<GraphicAdapter>>,
    pub input: Input,
    pub diagnostics: Diagnostics,
}

/*
pub trait WaitLoop {
}
*/
