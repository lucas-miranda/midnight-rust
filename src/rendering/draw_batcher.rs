use std::{slice::Iter, collections::HashMap};
use crate::math::Vector2;
use super::{
    backend::DrawCommand,
    shaders::{
        ShaderId,
        ShaderInstance,
    },
    DrawConfig,
    RenderState, ShaderConfig,
};

#[derive(Default)]
pub struct DrawBatcher<'a> {
    //vertex_buffer: Vec<Vector2<f32>>,
    batches: HashMap<ShaderId, ShaderBatch<'a>>,
}

impl<'a> DrawBatcher<'a> {
    /*
    pub fn drain(&mut self) -> impl Iterator<Item = Vector2<f32>> + '_ {
        self.vertex_buffer.drain(..)
    }
    */

    pub fn register_shader<S: ShaderInstance>(&mut self, shader: &'a S) {
        self.batches.insert(
            shader.id(),
            ShaderBatch {
                instance: shader,
                groups: Default::default(),
            }
        );
    }

    pub fn flush(&mut self, draw_command: &mut DrawCommand) {
        println!("-> Flushing...");
        for (shader_id, batch) in self.batches.drain() {
            println!("-> With shader id {}", shader_id);
            for (config, group) in batch.groups {
                println!("-> Group");
                let mut pass = draw_command.begin(batch.instance, &config, None);
                println!("Vertex count: {}", group.vertices.len());
                pass.extend(group.vertices.iter(), DrawConfig::EMPTY);
                pass.submit().unwrap();
            }
        }
    }
}

impl<'a> RenderState for DrawBatcher<'a> {
    fn extend(&mut self, vertices: Iter<Vector2<f32>>, config: DrawConfig) {
        let shader_config = config.shader_config
                                  .expect("Expecting shader config to be defined at this point.");

        let shader_id = shader_config.shader_id();

        if !self.batches.contains_key(shader_id) {
            panic!("Shader with id {} isn't registered.", shader_id);
        }

        let shader_batch = self.batches.get_mut(shader_id).unwrap();

        let batch_group = match shader_batch.groups.get_mut(&shader_config) {
            Some(group) => group,
            None => {
                shader_batch.groups.insert(shader_config, Default::default());
                shader_batch.groups.get_mut(&shader_config).unwrap()
            },
        };

        batch_group.vertices.extend(vertices.map(
            |v| *v + config.position
        ));
    }
}

struct ShaderBatch<'a> {
    pub instance: &'a dyn ShaderInstance,
    pub groups: HashMap<ShaderConfig, BatchGroup>,
}

#[derive(Default)]
struct BatchGroup {
    pub vertices: Vec<Vector2<f32>>,
}
