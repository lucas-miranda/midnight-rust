use std::error::Error;

use crate::window::WindowContext;

use super::{
    ApplicationError,
    ApplicationLoop,
    ApplicationState,
    InitFn,
    InitState,
};

#[derive(Default)]
pub struct Application {
    init_fn: Option<InitFn>,
}

impl Application {
    pub fn run<L: 'static + ApplicationLoop>(&mut self) -> Result<(), ApplicationError> {
        Self::display_header();

        let loop_control = L::new(
                WindowContext::new()
                    .map_err(ApplicationError::WindowCreationFailed)?
            );

        loop_control.run(self.init_fn.take())?;

        Ok(())
    }

    pub fn initialize_with<E, F>(mut self, init_fn: F) -> Self where
        E: 'static + Error,
        F: 'static + FnOnce(&mut ApplicationState, InitState) -> Result<(), E>,
    {
        self.init_fn = Some(Box::new(|app_state, init_state| {
            init_fn(app_state, init_state).map_err(Into::into)
        }));

        self
    }

    fn display_header() {
        let cargo_pkg_name = env!("CARGO_PKG_NAME");
        let cargo_pkg_version = env!("CARGO_PKG_VERSION");

        let border_len = cargo_pkg_name.len().max(cargo_pkg_version.len() + 2);
        let border = "─".repeat(border_len);
        let pkg_name_fill = " ".repeat(border_len - cargo_pkg_name.len());
        let pkg_version_fill = " ".repeat(border_len - (cargo_pkg_version.len() + 1));

        println!(" ┌──{}──┐", border);
        println!(" │  {}{}  │", cargo_pkg_name, pkg_name_fill);
        println!(" │  {}v{}  │", pkg_version_fill, cargo_pkg_version);
        println!(" └──{}──┘", border);
        println!();
    }
}
