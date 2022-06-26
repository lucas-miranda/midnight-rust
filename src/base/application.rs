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
        println!(" ┌───────────┐");
        println!(" │  {}  │", env!("CARGO_PKG_NAME"));
        println!(" │   v{}  │", env!("CARGO_PKG_VERSION"));
        println!(" └───────────┘");
        println!();
    }
}
