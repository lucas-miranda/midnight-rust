mod application;
pub use application::Application;

mod application_error;
pub use application_error::ApplicationError;

use std::{rc::Rc, cell::RefCell};
use crate::{
    ecs::Domain,
    rendering::GraphicAdapter,
    time::Time,
    window::{Window, WindowContext},
};

const WINDOW_SIZE: [u32; 2] = [512, 512];


pub struct ApplicationState {
    pub main_window: Window,
    pub time: Time,
    // TODO  is using Rc<RefCell<>> here a good solution?
    pub graphic_adapter: Rc<RefCell<GraphicAdapter>>,
}

pub trait ApplicationLoop {
    fn new(window_context: WindowContext) -> Self;
    fn register_domain<D: Into<Box<dyn Domain>>>(&mut self, domain: D);
    fn run(self);
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
        let graphic_adapter = GraphicAdapter::with_surface_size(
                &window,
                (physical_window_size.width, physical_window_size.height),
            )
            .unwrap();

        println!(
            "Window started with (Logical: {:?}, Physical: {:?}",
            logical_window_size,
            physical_window_size,
        );

        // compose application state
        let mut state = ApplicationState {
            main_window: window,
            time,
            graphic_adapter: Rc::new(RefCell::new(graphic_adapter)),
        };

        // setup all domains
        for domain in &mut self.domains {
            domain.setup(&mut state, &mut self.window_context);
        }

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
                    state.graphic_adapter.borrow_mut().request_resize_surface(dims.width, dims.height);
                }
                winit::event::Event::MainEventsCleared => {
                    let delta_time = state.time.delta(&mut last_instant);

                    for domain in &mut self.domains {
                        domain.update(&delta_time);
                    }

                    state.main_window.request_redraw();
                },
                winit::event::Event::RedrawRequested(_) => {
                    for domain in &mut self.domains {
                        domain.render();
                    }

                    //state.graphic_adapter.borrow_mut().render();
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
