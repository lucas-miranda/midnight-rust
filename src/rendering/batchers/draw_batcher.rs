use std::{
    cell::{ Ref, RefMut, RefCell },
    slice::Iter,
    rc::Rc,
};

use crate::{rendering::{
    backend::DrawCommand,
    shaders::{ Shader, ShaderInstance },
    texture::TextureId,
    DrawConfig,
    RenderState,
    RenderStateError,
    ShaderConfig,
    Texture,
    TextureConfig,
    TextureView,
    Vertex,
}, resources::AssetWeak};

use super::{DrawBatcherError, BatchMode};

pub struct DrawBatcher<'a, 'r, V: Vertex> {
    batches: Vec<ShaderBatch<'a, V>>,
    draw_command: &'a mut DrawCommand<'r>,
    mode: BatchMode,
}

impl<'a, 'r, V: Vertex> DrawBatcher<'a, 'r, V> {
    pub fn new(draw_command: &'a mut DrawCommand<'r>) -> Self {
        Self {
            batches: Vec::default(),
            draw_command,
            mode: BatchMode::DrawOrder,
        }
    }

    pub fn batch_count(&self) -> usize {
        self.batches.len()
    }

    pub fn shader_instances<'i>(
        &'i self
    ) -> impl Iterator<Item = Ref<'i, (dyn ShaderInstance + 'static)>> {
        self.batches.iter().map(|b| b.instance.borrow())
    }

    pub fn mut_shader_instances<'i>(
        &'i self
    ) -> impl Iterator<Item = RefMut<'i, (dyn ShaderInstance + 'static)>> {
        self.batches.iter().map(|b| b.instance.borrow_mut())
    }

    pub fn flush(mut self) -> Result<(), DrawBatcherError> {
        //println!("-> Flushing ({} batches)...", self.batches.len());
        for batch in self.batches.drain(..) {
            let shader_id = batch.instance.borrow().identifier();
            //println!("-> With shader id {}", shader_id);
            let (_texture_id, shader_config, _texture_config) = batch.group.configuration;

            //println!("-> Group");
            let shader = batch.instance.borrow();
            let mut pass = self.draw_command.begin(&shader, &shader_config, None)?;

            {
                let bindings = pass.bindings();

                if let Some(texture_view) = batch.group.texture_view {
                    //println!("-> With texture ({})", texture_view.id);
                    bindings.texture_view(texture_view)
                            .map_err(|e| DrawBatcherError::Bindings(e, shader_id))?;
                } else {
                    //println!("-> Without texture");
                }
            }

            //println!("Vertex count: {}", batch.group.vertices.len());
            pass.extend(
                batch.group.vertices.iter(),
                None,
                DrawConfig {
                    vertex: V::default(),
                    shader_config: None,
                    texture_config: None,
                }
            ).map_err(DrawBatcherError::from)?;

            pass.submit().map_err(DrawBatcherError::from)?;
        }

        //println!("----------------\n");
        Ok(())
    }

    fn create_batch<'t>(
        &mut self,
        shader: &Shader,
        texture: Option<AssetWeak<Texture>>,
        configuration: (TextureId, ShaderConfig, TextureConfig),
    ) -> Result<&mut ShaderBatch<'a, V>, RenderStateError> {
        let weak_instance = match self.draw_command.shader_builder().get_instance(shader) {
            Some(ins) => ins,
            None => return Err(RenderStateError::ShaderInstanceNotFound(shader.clone())),
        };

        self.batches.push(ShaderBatch {
            instance: weak_instance.upgrade()
                    .expect(format!("Shader ({}) was dropped", shader).as_str()),
            group: BatchGroup::new(
                texture.map(|t| {
                    let tex_asset = t.upgrade().expect("Failed to get texture");
                    let tex = tex_asset.get();
                    let (device, queue) = self.draw_command.device_queue();
                    tex.view(device, queue, configuration.2.clone())
                }),
                configuration,
            ),
        });

        // NOTE  safe to unwrap, we just pushed an element
        Ok(self.batches.last_mut().unwrap())
    }
}

impl<'a, 'r, V> RenderState<V> for DrawBatcher<'a, 'r, V> where
    V: Vertex,
{
    fn extend<'t>(
        &mut self,
        vertices: Iter<V>,
        texture: Option<AssetWeak<Texture>>,
        draw_config: DrawConfig<V>
    ) -> Result<(), RenderStateError> {
        let shader_config = draw_config
                             .shader_config
                             .ok_or_else(|| RenderStateError::MissingShaderConfig)?;

        let shader = shader_config.shader();

        match self.draw_command.shader_builder().get_context(shader) {
            Some(c) => {
                // check if `draw_config.texture_config` violates shader bindings requirements
                // TODO  maybe we can disable these validations at release mode

                for b in c.bindings_descriptor() {
                    b.validate_config(&draw_config)
                     .map_err(RenderStateError::from)?;
                }

                Ok::<_, RenderStateError>(())
            },
            None => return Err(RenderStateError::ShaderNotFound(shader.clone()))
        }?;

        let texture_config = draw_config.texture_config.unwrap_or_default();

        let texture_id = match texture {
            Some(ref t) => t.upgrade().map_or_else(|| TextureId::NONE, |v| v.get().id().clone()),
            None => TextureId::NONE,
        };

        // check if we can extend last batch
        // or we'll need a new one
        let configuration = (texture_id.clone(), shader_config.clone(), texture_config.clone());

        let shader_batch = match self.batches.last_mut() {
            Some(last_batch) => if last_batch.group.configuration == configuration {
                // we can reuse last batch
                last_batch
            } else {
                // we need a new one
                self.create_batch(shader, texture, configuration)?
            },
            None => self.create_batch(shader, texture, configuration)?,
        };

        shader_batch.group.vertices.extend(vertices.map(
            |v| *v + draw_config.vertex
        ));

        Ok(())
    }
}

struct ShaderBatch<'a, V: Vertex> {
    pub instance: Rc<RefCell<dyn ShaderInstance>>,
    pub group: BatchGroup<'a, V>,
}

#[derive(Default)]
struct BatchGroup<'v, V: Vertex> {
    pub texture_view: Option<TextureView<'v>>,
    pub configuration: (TextureId, ShaderConfig, TextureConfig),
    pub vertices: Vec<V>,
}

impl<'v, V: Vertex> BatchGroup<'v, V> {
    pub fn new(
        texture_view: Option<TextureView<'v>>,
        configuration: (TextureId, ShaderConfig, TextureConfig),
    ) -> Self {
        Self {
            texture_view,
            configuration,
            vertices: Vec::new(),
        }
    }
}
