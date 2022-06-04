use crate::{ecs::Domain, window::WindowContext};
use super::{ApplicationError, ApplicationLoop};

#[derive(Default)]
pub struct Application {
    domains: Vec<Box<dyn Domain>>
}

impl Application {
    pub fn run<L: 'static + ApplicationLoop>(&mut self) -> Result<(), ApplicationError> {
        /*
        for domain in &mut self.domains {
            domain.setup();
        }
        */

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
}
