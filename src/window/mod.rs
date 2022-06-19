use winit::{
    dpi::{LogicalSize, PhysicalSize},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
};

pub struct WindowEventHandler {
    control_flow: ControlFlow,
}

impl WindowEventHandler {
    pub fn new() -> Self {
        Self {
            control_flow: ControlFlow::Poll,
        }
    }

    pub fn request_close(&mut self) {
        self.control_flow = ControlFlow::Exit
    }
}

pub struct WindowContext {
    event_loop: EventLoop<()>,
    event_handler: WindowEventHandler,
}

impl WindowContext {
    pub fn new() -> Self {
        Self {
            event_loop: EventLoop::new(),
            event_handler: WindowEventHandler::new(),
        }
    }

    pub fn create_window(&self) -> WindowBuilder {
        WindowBuilder::new(&self)
    }

    pub fn calculate_window_size(
        &self,
        target_size: (u32, u32)
    ) -> (LogicalSize<u32>, PhysicalSize<u32>) {
        let dpi = match self.event_loop.primary_monitor() {
            Some(primary) => primary.scale_factor(),
            None => 1f64,
        };

        let logical: LogicalSize<u32> = target_size.into();
        let physical: PhysicalSize<u32> = logical.to_physical(dpi);

        (logical, physical)
    }

    pub fn run<F: 'static + FnMut(winit::event::Event<()>, &mut WindowEventHandler)>(mut self, mut handler: F) {
        self.event_loop.run(move |event, _event_window_target, control_flow| {
            handler(event, &mut self.event_handler);
            *control_flow = self.event_handler.control_flow;
        });
    }
}

pub struct WindowBuilder<'a> {
    context: &'a WindowContext,
    builder: winit::window::WindowBuilder,
}

impl<'a> WindowBuilder<'a> {
    pub(super) fn new(context: &'a WindowContext) -> Self {
        Self {
            context,
            builder: winit::window::WindowBuilder::new(),
        }
    }

    pub fn with_inner_size<S: Into<winit::dpi::Size>>(mut self, size: S) -> Self {
        self.builder = self.builder.with_inner_size(size);
        self
    }

    pub fn build(self) -> Result<Window, &'static str> {
        match self.builder.build(&self.context.event_loop) {
            Ok(w) => {
                Ok(Window::new(w))
            },
            Err(_e) => {
                Err("Failed to build window")
                //Err(&format!("{}", e.to_string()))
            }
        }
    }
}

pub struct Window {
    internal_window: winit::window::Window,
}

impl Window {
    pub(super) fn new(internal_window: winit::window::Window) -> Self {
        Self {
            internal_window,
        }
    }

    pub fn request_redraw(&self) {
        self.internal_window.request_redraw();
    }

    pub(crate) fn internal_window(&self) -> &winit::window::Window {
        &self.internal_window
    }
}
