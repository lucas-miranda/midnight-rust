use super::component::ComponentHandlerContainer;

pub trait System {
    type Container: ComponentHandlerContainer + 'static;

    fn setup(&mut self);
    fn run(&mut self, container: &mut Self::Container);
    fn create_container(&self) -> Self::Container;
}
