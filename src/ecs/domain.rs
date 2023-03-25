use crate::{
    base::ApplicationState,
    time::DeltaTime,
    window::WindowContext,
};

/// A Domain is a set of Systems
pub trait Domain {
    fn setup(&mut self, app_state: &mut ApplicationState, window_context: &mut WindowContext);
    fn input(&mut self, event: &winit::event::DeviceEvent);
    fn update(&mut self, delta: &DeltaTime);
    fn render(&mut self);
}
