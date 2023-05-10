mod default_shader;
pub use default_shader::*;

use std::{
    rc::{Rc, Weak},
    cell::RefCell
};

use crate::{
    components::{
        transform::Transform,
        GraphicDisplayer,
    },
    ecs::{
        component::{
            self,
            BaseQuery,
            QueryEntry,
        },
        system::System,
    },
    input,
    math::Matrix4x4,
    rendering::{
        shaders::{
            builder::ShaderFormat,
            AttributeFormat,
            VertexAttribute,
        },
        Color,
        DrawBatcher,
        DrawConfig,
        GraphicAdapter,
    },
    vertex_attrs,
};

pub struct RenderSystem {
    graphic_adapter: Weak<RefCell<GraphicAdapter>>,
    default_shader: DefaultShader,
}

impl RenderSystem {
    pub fn new(graphic_adapter: &Rc<RefCell<GraphicAdapter>>) -> Self {
        let default_shader = graphic_adapter
            .borrow_mut()
            .shader_builder()
            .create::<DefaultUniforms>(
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
            default_shader,
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


        {
            let mut uniforms = self.default_shader.uniforms_mut();
            uniforms.view = Matrix4x4::ortho(0.0, 180.0, 0.0, 320.0, -100.0, 100.0);
            uniforms.color = Color::<f32>::rgb_hex(0x0000FF);
        }

        let mut adapter = graphic_adapter.borrow_mut();

        match adapter.prepare_draw() {
            Ok(mut draw_command) => {
                // TODO  use default shader to clear screen
                draw_command.clear(Color::<u8>::rgb_hex(0x46236E), &self.default_shader)
                            .unwrap();

                // collects everything indo a batcher
                let mut draw_batcher = DrawBatcher::default();
                draw_batcher.register_shader(&self.default_shader);

                for QueryEntry { component: (a, b), .. } in query.iter_components() {
                    if let Some(graphic_displayer) = a {
                        if let Some(transform) = b {
                            if let Some(ref g) = graphic_displayer.graphic {
                                let draw_config = DrawConfig {
                                    position: transform.position(),
                                    shader_config: graphic_displayer
                                                    .shader_config
                                                    .or_else(|| { Some(
                                                        self.default_shader
                                                            .default_config()
                                                            .clone()
                                                    ) } ),
                                };

                                println!("[RenderSystem] Rendering with {:?}", draw_config);
                                println!("[RenderSystem] Transform: {:?}", *transform);

                                g.draw(&mut draw_batcher, draw_config)
                            }
                        }
                    }
                }

                draw_batcher.flush(&mut draw_command);
                draw_command.present();
            },
            Err(_e) => return,
        }
    }

    fn create_query<'q>(&self) -> Self::Query<'q> {
        Self::Query::default()
    }
}
