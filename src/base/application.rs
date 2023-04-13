use crate::{ecs::Domain, window::WindowContext};
use super::{ApplicationError, ApplicationLoop};

#[derive(Default)]
pub struct Application {
    domains: Vec<Box<dyn Domain>>
}

impl Application {
    pub fn run<L: 'static + ApplicationLoop>(&mut self) -> Result<(), ApplicationError> {
        Self::display_header();
        let mut loop_control = L::new(WindowContext::new());

        for domain in self.domains.drain(..) {
            loop_control.register_domain(domain);
        }

        loop_control.run();

        Ok(())
    }

    pub fn with_domain<D: 'static + Domain>(mut self, domain: D) -> Self {
        self.domains.push(Box::new(domain));
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
