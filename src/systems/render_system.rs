use std::{
    rc::{Rc, Weak},
    cell::RefCell
};

use bytemuck::{Pod, Zeroable};

use crate::{
    components::GraphicDisplayer,
    ecs::{
        component::{self, ComponentQuery},
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
    vertex_attrs, math::{Matrix4x4, Vector4},
};

//

#[derive(Copy, Clone, Default, Pod, Zeroable)]
#[repr(C)]
struct MyUniforms {
    pub view: Matrix4x4<f32>,
    pub color: Vector4<f32>,
}

//

pub struct RenderSystem {
    graphic_adapter: Weak<RefCell<GraphicAdapter>>,
    shader: Shader,
}

impl RenderSystem {
    pub fn new(graphic_adapter: &Rc<RefCell<GraphicAdapter>>) -> Self {
        let shader = graphic_adapter
            .borrow_mut()
            .shader_builder()
            .create::<MyUniforms>(
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
        }
    }
}

impl System for RenderSystem {
    type Query<'q> = component::Query<'q, GraphicDisplayer>;

    fn setup(&mut self) {
    }

    fn input<'q>(&mut self, _query: Self::Query<'q>, _event: &input::DeviceEvent) {
    }

    fn run<'q>(&mut self, query: Self::Query<'q>) {
        println!("[RenderSystem] {} captured components", query.count());
        let graphic_adapter = self.graphic_adapter.upgrade().unwrap();

        let my_uniforms = MyUniforms {
            view: Matrix4x4::ortho(0.0, 180.0, 0.0, 320.0, -100.0, 100.0),
            color: Vector4::new(1.0, 0.0, 1.0, 1.0)
        };

        for component_ref in query.iter_components() {
            if let Some(ref g) = component_ref.graphic {
                let mut adapter = graphic_adapter.borrow_mut();

                g.draw(&mut adapter)
                 .using_shader(&self.shader, Some(&my_uniforms))
                 .submit()
                 .unwrap();
            }
        }
    }

    fn create_query<'q>(&self) -> Self::Query<'q> {
        Self::Query::default()
    }
}
