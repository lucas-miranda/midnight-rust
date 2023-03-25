use std::{
    rc::{Rc, Weak},
    cell::RefCell
};

use crate::{
    components::GraphicDisplayer,
    ecs::{
        component,
        system::System,
    },
    input,
    rendering::{
        shaders::{
            builder::ShaderFormat,
            AttributeFormat,
            Shader,
            VertexAttribute,
        },
        GraphicAdapter,
    },
    vertex_attrs,
};

pub struct RenderSystem<'a> {
    graphic_adapter: Weak<RefCell<GraphicAdapter>>,
    shader: Shader,
    phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a> RenderSystem<'a> {
    pub fn new(graphic_adapter: &Rc<RefCell<GraphicAdapter>>) -> Self {
        let shader = graphic_adapter
            .borrow_mut()
            .shader_builder()
            .create(
                ShaderFormat::GLSL,
                include_str!("shaders/p1.vert"),
                include_str!("shaders/p1.frag"),
            )
            .set_vertex_attributes(vertex_attrs![
                Float32x2,
            ].into_iter())
            .build();

        Self {
            graphic_adapter: Rc::downgrade(graphic_adapter),
            shader,
            phantom: Default::default(),
        }
    }
}

impl<'a> System for RenderSystem<'a> {
    type Query = component::Query<'a, GraphicDisplayer>;

    fn setup(&mut self) {
    }

    fn input(&mut self, _query: Self::Query, _event: &input::DeviceEvent) {
    }

    fn run(&mut self, query: Self::Query) {
        println!("[RenderSystem] {} captured components", query.count());
        let graphic_adapter = self.graphic_adapter.upgrade().unwrap();

        for component_ref in query.iter() {
            if let Some(ref g) = (*component_ref.borrow_value()).graphic {
                g.draw(graphic_adapter.borrow_mut(), &self.shader);
            }

            /*
            if let Ok(displayer) = component_ref.retrieve() {
                if let Some(ref g) = (*displayer.borrow_value()).graphic {
                    g.draw(graphic_adapter.borrow_mut(), &self.shader);
                }
            }
            */

            //

            /*
            if let Some(ref displayer) = *component_ref.as_deref() {
                if let Some(ref g) = displayer.borrow().graphic {
                    g.draw(graphic_adapter.borrow_mut(), &self.shader);
                }
            }
            */
        }
    }

    fn create_query(&self) -> Self::Query {
        Self::Query::default()
    }
}
