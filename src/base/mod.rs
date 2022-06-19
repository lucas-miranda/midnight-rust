mod application;
pub use application::Application;

mod application_error;
pub use application_error::ApplicationError;

use crate::{
    ecs::Domain,
    rendering::GraphicAdapter,
    time::Time,
    window::WindowContext,
};

const WINDOW_SIZE: [u32; 2] = [512, 512];

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
        let (logical_window_size, physical_window_size)
            = self.window_context.calculate_window_size(
                (WINDOW_SIZE[0], WINDOW_SIZE[1])
            );

        // window
        let window = self.window_context
            .create_window()
            .with_inner_size(logical_window_size)
            .build()
            .unwrap();

        // time
        let time = Time::new();
        let mut last_instant = Time::now();

        // rendering
        let mut graphic_adapter = GraphicAdapter::with_surface_size(
                &window,
                (physical_window_size.width, physical_window_size.height),
            )
            .unwrap();

        println!("Window started with (Logical: {:?}, Physical: {:?}", logical_window_size, physical_window_size);
        //render_backend.request_configure_swapchain_with(physical_window_size.width, physical_window_size.height);

        self.window_context.run(move |event, event_handler| {
            match event {
                winit::event::Event::WindowEvent {
                    event: winit::event::WindowEvent::CloseRequested, ..
                } => {
                    println!("Window closed by user");
                    event_handler.request_close();
                },
                winit::event::Event::WindowEvent {
                    event: winit::event::WindowEvent::Resized(dims), ..
                } => {
                    println!("Window resized to {:?}", dims);
                    graphic_adapter.request_resize_surface(dims.width, dims.height);
                }
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

                    graphic_adapter.render();
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
