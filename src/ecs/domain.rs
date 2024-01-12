use crate::{
    base::ApplicationState,
    window::WindowContext,
};

use super::FrameState;

/// A Domain is a set of Systems
pub trait Domain {
    fn setup(&mut self, app_state: &mut ApplicationState, window_context: &mut WindowContext);
    fn input(&mut self, app_state: &mut ApplicationState);
    fn update<'u>(&'u mut self, state: &'u mut FrameState);
    fn render<'r>(&'r mut self, state: &'r mut FrameState);
}
