use std::{rc::{Rc, Weak}, cell::RefCell};

use crate::{
    components::GraphicDisplayer,
    ecs::{
        component::ComponentContainer,
        System,
    },
    rendering::{
        shaders::{builder::ShaderFormat, Shader},
        GraphicAdapter,
    },
};

pub struct RenderSystem {
    graphic_adapter: Weak<RefCell<GraphicAdapter>>,
    shader: Shader,
}

impl RenderSystem {
    pub fn new(graphic_adapter: &Rc<RefCell<GraphicAdapter>>) -> Self {
        let shader = graphic_adapter
            .borrow_mut()
            .shader_builder()
            .build(
                ShaderFormat::GLSL,
                include_str!("shaders/p1.vert"),
                include_str!("shaders/p1.frag"),
            );

        Self {
            graphic_adapter: Rc::downgrade(graphic_adapter),
            shader,
        }
    }
}

impl System for RenderSystem {
    type Container = ComponentContainer<GraphicDisplayer>;

    fn setup(&mut self) {
    }

    fn run(&mut self, container: &mut Self::Container) {
        println!("[RenderSystem] {} captured components", container.count());
        let graphic_adapter = self.graphic_adapter.upgrade().unwrap();

        for component_ref in container.iter() {
            if let Some(ref displayer) = *component_ref.as_deref() {
                if let Some(ref g) = displayer.graphic {
                    g.draw(graphic_adapter.borrow_mut(), &self.shader);
                }
            }
        }
    }

    fn create_container(&self) -> Self::Container {
        Self::Container::default()
    }
}
