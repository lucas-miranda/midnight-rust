mod application;
pub use application::Application;

mod application_error;
pub use application_error::ApplicationError;

use crate::{
    ecs::Domain,
    time::Time,
    window::WindowContext,
};

pub trait ApplicationLoop {
    fn new(window_context: WindowContext) -> Self;
    fn register_domain<D: Into<Box<dyn Domain>>>(&mut self, domain: D);
    fn run(self);
    //fn execute(&mut self, event: Event<()>, event_window_target: &EventLoopWindowTarget<()>, control_flow: &mut ControlFlow);
}

pub struct ContinuousLoop {
    window_context: WindowContext,
    domains: Vec<Box<dyn Domain>>,
}

impl ContinuousLoop {
}

impl ApplicationLoop for ContinuousLoop {
    fn new(window_context: WindowContext) -> Self {
         Self {
            window_context,
            domains: Vec::new(),
        }
    }

    fn register_domain<D: Into<Box<dyn Domain>>>(&mut self, domain: D) {
        self.domains.push(domain.into());
    }

    fn run(mut self) {
        let window = self.window_context
            .create_window()
            .build()
            .unwrap();

        let time = Time::new();
        let mut last_instant = Time::instant();

        self.window_context.run(move |event, event_handler| {
            match event {
                winit::event::Event::WindowEvent {
                    event: winit::event::WindowEvent::CloseRequested, ..
                } => {
                    println!("Windows closed by user");
                    event_handler.request_close();
                },
                winit::event::Event::MainEventsCleared => {
                    let delta_time = time.delta(&mut last_instant);

                    for domain in &mut self.domains {
                        domain.update(&delta_time);
                    }

                    window.request_redraw();
                },
                winit::event::Event::RedrawRequested(_) => {
                    for domain in &mut self.domains {
                        domain.render();
                    }
                },
                _ => ()
            }
        });
    }
}

/*
pub trait WaitLoop {
}
*/
