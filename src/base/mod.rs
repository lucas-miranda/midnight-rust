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
    rendering::{GraphicAdapter, Texture},
    resources::AssetResources,
    time::Time,
    window::Window,
};

pub struct ApplicationState {
    pub main_window: Window,
    pub time: Time,
    pub graphic_adapter: Rc<RefCell<GraphicAdapter>>,
    pub input: Input,
    pub diagnostics: Diagnostics,
    pub asset_resources: AssetResources,
}

impl ApplicationState {
    fn new(window: Window, graphic_adapter: GraphicAdapter) -> Self {
        let mut asset_resources = AssetResources::default();

        asset_resources.register_loader::<Texture>();

        Self {
            main_window: window,
            time: Time::new(),
            graphic_adapter: Rc::new(RefCell::new(graphic_adapter)),
            input: Input::default(),
            diagnostics: Default::default(),
            asset_resources,
        }
    }
}
