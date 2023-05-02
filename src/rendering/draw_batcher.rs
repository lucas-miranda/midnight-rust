use std::{slice::Iter, collections::HashMap};
use crate::math::Vector2;
use super::{
    backend::DrawCommand,
    shaders::{
        ShaderId,
        ShaderInstance,
    },
    DrawConfig,
    RenderState,
};

#[derive(Default)]
pub struct DrawBatcher<'a> {
    //vertex_buffer: Vec<Vector2<f32>>,
    batches: HashMap<ShaderId, BatchGroup<'a>>,
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
            BatchGroup {
                vertices: Vec::default(),
                shader_instance: shader,
            }
        );
    }

    pub fn flush(&mut self, draw_command: &mut DrawCommand) {
        for (_shader_id, group) in self.batches.drain() {
            let mut pass = draw_command.begin(group.shader_instance, None);
            pass.extend(group.vertices.iter(), DrawConfig::EMPTY);
            pass.submit().unwrap();
        }
    }
}

impl<'a> RenderState for DrawBatcher<'a> {
    fn extend(&mut self, vertices: Iter<Vector2<f32>>, config: DrawConfig) {
        /*
        self.vertex_buffer.extend(
            vertices.map(|v| *v + draw_config.position)
        );
        */

        /*
        if !self.batches.contains_key(&config.shader_id) {
            self.batches.insert(config.shader_id, BatchGroup::default());
        }
        */

        if !self.batches.contains_key(&config.shader_id) {
            panic!("Shader with id {} isn't registered.", config.shader_id);
        }

        let batch_group = self.batches.get_mut(&config.shader_id).unwrap();

        batch_group.vertices.extend(vertices.map(
            |v| *v + config.position
        ));

        /*
        batch_group.entries.push(
            BatchEntry {
                vertices: vertices.map(|v| *v + config.position).collect(),
                config,
            }
        );
        */
    }
}

struct BatchEntry {
    pub vertices: Vec<Vector2<f32>>,
    pub config: DrawConfig,
}

struct BatchGroup<'a> {
    pub vertices: Vec<Vector2<f32>>,
    //pub entries: Vec<BatchEntry>,
    pub shader_instance: &'a dyn ShaderInstance,
}
