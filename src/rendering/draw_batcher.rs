use std::{
    cell::{ Ref, RefMut, RefCell },
    collections::HashMap,
    slice::Iter,
    rc::Rc,
};

use super::{
    backend::DrawCommand,
    shaders::{ Shader, ShaderInstance },
    texture::TextureId,
    DrawBatcherError,
    DrawConfig,
    RenderState,
    RenderStateError,
    ShaderConfig,
    Texture,
    TextureView,
    Vertex, TextureConfig,
};

pub struct DrawBatcher<'a, 'r, V: Vertex> {
    batches: HashMap<Shader, ShaderBatch<'a, V>>,
    draw_command: &'a mut DrawCommand<'r>,
}

impl<'a, 'r, V: Vertex> DrawBatcher<'a, 'r, V> {
    pub fn new(draw_command: &'a mut DrawCommand<'r>) -> Self {
        let mut batches = HashMap::default();

        // register every shader from shader builder
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

    pub fn shader_instances<'i>(
        &'i self
    ) -> impl Iterator<Item = Ref<'i, (dyn ShaderInstance + 'static)>> {
        self.batches.values().map(|b| b.instance.borrow())
    }

    pub fn mut_shader_instances<'i>(
        &'i self
    ) -> impl Iterator<Item = RefMut<'i, (dyn ShaderInstance + 'static)>> {
        self.batches.values().map(|b| b.instance.borrow_mut())
    }

    pub fn flush(mut self) -> Result<(), DrawBatcherError> {
        //println!("-> Flushing...");
        for (_shader_id, batch) in self.batches.drain() {
            //println!("-> With shader id {}", _shader_id);
            for ((_texture_id, shader_config, texture_config), group) in batch.groups {
                //println!("-> Group");
                let shader = batch.instance.borrow();
                let mut pass = self.draw_command.begin(&shader, &shader_config, None)?;

                {
                    let bindings = pass.bindings();

                    if let Some(texture_view) = group.texture_view {
                        //println!("-> With texture ({})", texture_view.id);
                        bindings.texture_view(texture_view).map_err(DrawBatcherError::from)?;
                    }
                }

                //println!("Vertex count: {}", group.vertices.len());
                pass.extend(
                    group.vertices.iter(),
                    None,
                    DrawConfig {
                        vertex: V::default(),
                        shader_config: None,
                        texture_config: None,
                    }
                ).map_err(DrawBatcherError::from)?;

                pass.submit().map_err(DrawBatcherError::from)?;
            }
        }

        //println!("----------------\n");
        Ok(())
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
    ) -> Result<(), RenderStateError> {
        let texture_config = draw_config.texture_config.unwrap_or_default();
        let shader_config = draw_config
                             .shader_config
                             .ok_or_else(|| RenderStateError::MissingShaderConfig)?;

        let shader = shader_config.shader();

        let texture_id = match texture {
            Some(t) => t.id(),
            None => &TextureId::NONE,
        };

        let shader_batch = self.batches
                               .get_mut(shader)
                               .ok_or_else(|| RenderStateError::ShaderNotFound(*shader))?;

        let batch_group = match shader_batch.groups.get_mut(&(*texture_id, shader_config, texture_config)) {
            Some(group) => {
                //println!("Already exists...");
                group
            },
            None => {
                //println!("Creating a new one...");
                shader_batch.groups.insert(
                    (*texture_id, shader_config, texture_config),
                    BatchGroup {
                        texture_view: texture.map(|t| {
                            let (device, queue) = self.draw_command.device_queue();
                            t.view(device, queue, texture_config)
                        }),
                        vertices: Vec::new(),
                    }
                );

                // NOTE  safe to unwrap  key was inserted previously
                shader_batch.groups.get_mut(&(*texture_id, shader_config, texture_config)).unwrap()
            },
        };

        batch_group.vertices.extend(vertices.map(
            |v| *v + draw_config.vertex
        ));

        Ok(())
    }
}

struct ShaderBatch<'a, V: Vertex> {
    pub instance: Rc<RefCell<dyn ShaderInstance>>,
    pub groups: HashMap<(TextureId, ShaderConfig, TextureConfig), BatchGroup<'a, V>>,
}

#[derive(Default)]
struct BatchGroup<'v, V: Vertex> {
    pub texture_view: Option<TextureView<'v>>,
    //pub texture_id: Option<TextureId>,
    pub vertices: Vec<V>,
}
