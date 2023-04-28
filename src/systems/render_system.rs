use std::{
    rc::{Rc, Weak},
    cell::RefCell
};

use bytemuck::{Pod, Zeroable};

use crate::{
    components::{
        transform::Transform,
        GraphicDisplayer,
    },
    ecs::{
        component::{self, BaseQuery, QueryEntry},
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
        Color,
        DrawConfig,
        GraphicAdapter,
        graphics::Graphic,
    },
    vertex_attrs, math::{Matrix4x4, Vector4, Vector2, Tri},
};

//

#[derive(Copy, Clone, Default, Pod, Zeroable)]
#[repr(C)]
struct MyUniforms {
    pub view: Matrix4x4<f32>,
    pub color: Color<f32>,
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
    type Query<'q> = (
        component::Query<'q, GraphicDisplayer>,
        component::Query<'q, Transform>,
    );

    fn setup(&mut self) {
    }

    fn input<'q>(&mut self, _query: Self::Query<'q>, _event: &input::DeviceEvent) {
    }

    fn run<'q>(&mut self, query: Self::Query<'q>) {
        println!(
            "[RenderSystem] captured components({}): {} GraphicDisplayer, {} Transform",
            query.iter_components().count(),
            query.0.iter_components().count(),
            query.1.iter_components().count()
        );

        let graphic_adapter = self.graphic_adapter.upgrade().unwrap();

        let my_uniforms = MyUniforms {
            view: Matrix4x4::ortho(0.0, 180.0, 0.0, 320.0, -100.0, 100.0),
            color: Color::rgba(1.0, 0.0, 1.0, 1.0),
        };

        let mut adapter = graphic_adapter.borrow_mut();

        match adapter.begin_draw() {
            Ok(mut draw_command) => {
                //println!("clearing...");
                //draw_command.clear(Color::<u8>::rgb(70, 35, 110), &self.shader)
                /*
                draw_command.clear(Color::<u8>::rgb(255, 0, 0), &self.shader, Some(&my_uniforms))
                            .unwrap();
                */
                //println!("clearing done!");

                let draw_config = DrawConfig {
                    position: Vector2::new(50.0, 50.0),
                };

                println!("[RenderSystem] Rendering with {:?}", draw_config);
                //println!("[RenderSystem] Transform: {:?}", *transform);

                let g = Tri::new(Vector2::new(0.0, 0.0), Vector2::new(0.0, 0.0),  Vector2::new(0.0, 0.0));

                draw_command.begin(None);

                g.draw(&mut draw_command, &draw_config)
                 .clear_color(Color::<u8>::rgb(70, 35, 110))
                 .using_shader(&self.shader, Some(&my_uniforms))
                 .submit()
                 .unwrap();

                draw_command.end();

                /*
                for QueryEntry { component: (a, b), .. } in query.iter_components() {
                    if let Some(graphic_displayer) = a {
                        draw_command.begin(None);

                        if let Some(transform) = b {
                            if let Some(ref g) = graphic_displayer.graphic {
                                let draw_config = DrawConfig {
                                    position: transform.position(),
                                };

                                println!("[RenderSystem] Rendering with {:?}", draw_config);
                                println!("[RenderSystem] Transform: {:?}", *transform);

                                g.draw(&mut draw_command, &draw_config)
                                 .using_shader(&self.shader, Some(&my_uniforms))
                                 .submit()
                                 .unwrap();
                            }
                        }

                        draw_command.end();
                    }
                }
                */

                draw_command.present();
            },
            Err(_e) => return,
        }
    }

    fn create_query<'q>(&self) -> Self::Query<'q> {
        Self::Query::default()
    }
}
