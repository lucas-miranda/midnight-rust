pub mod default_shader;

use std::{
    rc::{Rc, Weak},
    cell::RefCell,
    marker::PhantomData,
};

use crate::{
    base::ApplicationState,
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
        FrameState,
    },
    math::{
        Matrix4x4,
        Vector2,
    },
    rendering::{
        batchers::DrawBatcher,
        Color,
        DrawConfig,
        GraphicAdapter,
        Vertex,
        VertexPosition,
    },
    resources::{Asset, AssetRef, AssetWeak},
};

use default_shader::DefaultShader;

pub struct RenderSystem<V: Vertex> {
    //pub world: Matrix4x4<f32>,
    pub view: Matrix4x4<f32>,
    //pub projection: Matrix4x4<f32>,

    graphic_adapter: Weak<RefCell<GraphicAdapter>>,
    default_shader: AssetWeak<DefaultShader>,
    clear_color: Color::<f32>,
    phantom: PhantomData<V>,
}

impl<V: Vertex> RenderSystem<V> {
    pub fn new(app_state: &mut ApplicationState) -> Self {
        app_state.asset_resources.register_loader::<DefaultShader>();
        let g = app_state.asset_resources
                         .get_mut_group::<DefaultShader>()
                         .unwrap();

        let asset: Asset<_> = DefaultShader::new(&app_state.graphic_adapter).into();
        let default_shader = asset.weak();
        g.register_asset("default", asset);

        Self {
            graphic_adapter: Rc::downgrade(&app_state.graphic_adapter),
            default_shader,
            phantom: Default::default(),
            clear_color: 0xFF236EFF.into(),
            view: Matrix4x4::default(),
        }
    }

    pub fn with_clear_color<C: Into<Color<f32>>>(mut self, color: C) -> Self {
        self.clear_color = color.into();

        self
    }
}

impl<V: Vertex + VertexPosition<Position = Vector2<f32>>> System for RenderSystem<V> {
    type Query<'q> = (
        component::Query<'q, GraphicDisplayer<V>>,
        component::Query<'q, Transform>,
    );

    fn setup(&mut self) {
    }

    fn input<'q>(&mut self, _query: Self::Query<'q>, _state: &mut ApplicationState) {
    }

    fn run<'q>(&mut self, query: Self::Query<'q>, state: &mut FrameState) {
        /*
        println!(
            "[RenderSystem] captured components({}): {} GraphicDisplayer, {} Transform",
            query.iter_components().count(),
            query.0.iter_components().count(),
            query.1.iter_components().count()
        );
        */

        let graphic_adapter = self.graphic_adapter.upgrade().unwrap();

        /*
        //if let Some(shader) = self.default_shader.upgrade().map(|s| s.get_mut()) {
        if let Some(mut shader) = self.default_shader.upgrade().get_mut() {
            let uniforms = shader.uniforms_mut();
            //uniforms.view = Matrix4x4::ortho(180.0, 0.0, 0.0, 320.0, -100.0, 100.0);

            //uniforms.view = self.view;

            uniforms.color = Color::<f32>::rgba_hex(0x0000FFFF);
        }
        */

        let mut adapter = graphic_adapter.borrow_mut();

        match adapter.prepare_draw() {
            Ok(mut draw_command) => {
                // TODO  use default shader to clear screen

                if let Some(shader) = self.default_shader.upgrade().get() {
                    //let shader: std::cell::Ref<dyn ShaderInstance> = self.default_shader.borrow();
                    //draw_command.clear::<_, Vertex2D, _>(Color::<u8>::rgb_hex(0x46236E), &shader)
                    draw_command.clear(self.clear_color, &shader)
                                .unwrap();
                }

                // collects everything into a batcher
                {
                    let mut draw_batcher = DrawBatcher::new(&mut draw_command);

                    //draw_batcher.register_shader(&self.default_shader);

                    //println!("{} query components", query.iter_components().count());
                    for QueryEntry { component: (a, b), .. } in query.iter_components() {
                        if let Some(graphic_displayer) = a {
                            if let Some(transform) = b {
                                if let Some(ref g) = graphic_displayer.graphic {
                                    let draw_config = DrawConfig {
                                        vertex: V::from_position(transform.position()),
                                        shader_config: graphic_displayer
                                                        .shader_config
                                                        .or_else(|| { Some(
                                                            self.default_shader
                                                                .upgrade()
                                                                .get()?
                                                                .default_config()
                                                                .clone()
                                                        ) } ),
                                        texture_config: graphic_displayer
                                                         .texture_config
                                                         .or_else(|| Some(Default::default()))
                                    };

                                    //println!("[RenderSystem] Rendering with {:?}", draw_config);
                                    //println!("[RenderSystem] Transform: {:?}", *transform);

                                    g.draw(&mut draw_batcher, draw_config).unwrap()
                                }
                            }
                        }
                    }

                    state.app.diagnostics.draw_calls += draw_batcher.batch_count();

                    {
                        // update world view projection matrices for every shader in-use

                        for mut shader_instance in draw_batcher.mut_shader_instances() {
                            if let Some(wvp) = shader_instance.mut_world_view_projection_uniforms() {
                                let view = wvp.mut_view();
                                *view = self.view;
                            }
                        }
                    }

                    draw_batcher.flush().unwrap();
                }

                draw_command.present();
            },
            Err(_e) => return,
        }
    }

    fn create_query<'q>(&self) -> Self::Query<'q> {
        Self::Query::default()
    }
}
