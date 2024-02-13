mod window_error;
pub use window_error::WindowError;

use raw_window_handle::{ HasRawDisplayHandle, HasRawWindowHandle };

use winit::{
    dpi::{ LogicalSize, PhysicalSize },
    event_loop::{ ControlFlow, EventLoop },
};

use crate::math::Size2;

pub struct WindowEventHandler {
    pub(super) close_requested: bool
}

impl WindowEventHandler {
    pub fn new() -> Self {
        Self {
            close_requested: false
        }
    }

    pub fn request_close(&mut self) {
        self.close_requested = true
    }
}

pub struct WindowContext {
    event_loop: EventLoop<()>,
    event_handler: WindowEventHandler,
}

impl WindowContext {
    pub fn new() -> Result<Self, winit::error::EventLoopError> {
        Ok(Self {
            event_loop: EventLoop::new()?,
            event_handler: WindowEventHandler::new(),
        })
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

    pub fn run<F>(mut self, mut handler: F) -> Result<(), WindowError>
        where F: 'static + FnMut(winit::event::Event<()>, &mut WindowEventHandler)
    {
        self.event_loop.set_control_flow(ControlFlow::Poll);
        self.event_loop.run(move |event, event_window_target| {
            handler(event, &mut self.event_handler);

            if self.event_handler.close_requested {
                event_window_target.exit();
            }
        }).map_err(WindowError::from)
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

    pub fn scale_factor(&self) -> f64 {
        self.internal_window.scale_factor()
    }

    pub fn inner_size(&self) -> Size2<u32> {
        self.internal_window
            .inner_size()
            .to_logical(self.scale_factor())
            .into()
    }
}

unsafe impl HasRawWindowHandle for Window {
    fn raw_window_handle(&self) -> raw_window_handle::RawWindowHandle {
        self.internal_window.raw_window_handle()
    }
}

unsafe impl HasRawDisplayHandle for Window {
    fn raw_display_handle(&self) -> raw_window_handle::RawDisplayHandle {
        self.internal_window.raw_display_handle()
    }
}
