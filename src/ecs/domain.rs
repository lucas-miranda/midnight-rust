use crate::time::DeltaTime;

/// A Domain is a set of Systems
pub trait Domain {
    fn setup(&mut self);
    fn update(&mut self, delta: &DeltaTime);
    fn render(&mut self);
}
