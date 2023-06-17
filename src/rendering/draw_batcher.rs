use std::{
    cell::RefCell,
    collections::HashMap,
    slice::Iter,
    rc::Rc,
};

use super::{
    backend::DrawCommand,
    shaders::{
        ShaderId,
        ShaderInstance,
    },
    texture::TextureId,
    DrawConfig,
    RenderState,
    ShaderConfig,
    Texture,
    TextureView,
    Vertex,
};

pub struct DrawBatcher<'a, 'r, V: Vertex> {
    batches: HashMap<ShaderId, ShaderBatch<'a, V>>,
    draw_command: &'a mut DrawCommand<'r>,
}

impl<'a, 'r, V: Vertex> DrawBatcher<'a, 'r, V> {
    pub fn new(draw_command: &'a mut DrawCommand<'r>) -> Self {
        let mut batches = HashMap::default();

        draw_command.shader_builder()
            .instances()
            .iter()
            .for_each(|(id, weak_ref)| {
                batches.insert(
                    *id,
                    ShaderBatch {
                        instance: weak_ref.upgrade().expect(format!("Shader (id {}) was dropped", id).as_str()),
                        groups: Default::default(),
                    }
                );
            });

        Self {
            batches,
            draw_command,
        }
    }

    /*
    pub fn register_shader<S: ShaderInstance>(&mut self, shader: &'a S) {
        self.batches.insert(
            shader.id(),
            ShaderBatch {
                instance: shader,
                groups: Default::default(),
            }
        );
    }
    */

    pub fn flush(mut self) {
        println!("-> Flushing...");
        for (shader_id, batch) in self.batches.drain() {
            println!("-> With shader id {}", shader_id);
            for ((_texture_id, config), group) in batch.groups {
                println!("-> Group");
                let shader = batch.instance.borrow();
                let mut pass = self.draw_command.begin(&shader, &config, None);

                {
                    let bindings = pass.bindings();

                    if let Some(texture_view) = group.texture_view {
                        println!("-> With texture ({})", texture_view.id);
                        bindings.texture_view(texture_view);
                    }
                }

                println!("Vertex count: {}", group.vertices.len());
                pass.extend(
                    group.vertices.iter(),
                    None,
                    DrawConfig {
                        vertex: V::default(),
                        shader_config: None,
                    }
                );
                pass.submit().unwrap();
            }
        }

        println!("----------------\n");
    }
}

impl<'a, 'r, V> RenderState<V> for DrawBatcher<'a, 'r, V> where
    V: Vertex,
{
    fn extend<'t>(
        &mut self,
        vertices: Iter<V>,
        texture: Option<&'t Texture>,
        draw_config: DrawConfig<V>
    ) {
        let shader_config = draw_config.shader_config
                                  .expect("Expecting shader config to be defined at this point.");

        let shader_id = shader_config.shader_id();

        let texture_id = match texture {
            Some(t) => t.id(),
            None => &TextureId::NONE,
        };

        if !self.batches.contains_key(shader_id) {
            panic!("Shader with id {} isn't registered.", shader_id);
        }

        let shader_batch = self.batches.get_mut(shader_id).unwrap();

        let batch_group = match shader_batch.groups.get_mut(&(*texture_id, shader_config)) {
            Some(group) => {
                println!("Already exists...");
                group
            },
            None => {
                println!("Creating a new one...");
                shader_batch.groups.insert(
                    (*texture_id, shader_config),
                    BatchGroup {
                        texture_view: texture.map(|t| {
                            let (device, queue) = self.draw_command.device_queue();
                            t.view(device, queue)
                        }),
                        vertices: Vec::new(),
                    }
                );

                shader_batch.groups.get_mut(&(*texture_id, shader_config)).unwrap()
            },
        };

        batch_group.vertices.extend(vertices.map(
            |v| *v + draw_config.vertex
        ));
    }
}

struct ShaderBatch<'a, V: Vertex> {
    pub instance: Rc<RefCell<dyn ShaderInstance>>,
    pub groups: HashMap<(TextureId, ShaderConfig), BatchGroup<'a, V>>,
}

#[derive(Default)]
struct BatchGroup<'v, V: Vertex> {
    pub texture_view: Option<TextureView<'v>>,
    //pub texture_id: Option<TextureId>,
    pub vertices: Vec<V>,
}
