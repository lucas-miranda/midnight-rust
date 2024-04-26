mod shader_config;
pub use shader_config::ShaderConfig;

mod texture_config;
pub use texture_config::*;

pub use wgpu::{
    Face,
    FrontFace,
    IndexFormat,
    PolygonMode,
    PrimitiveState,
    PrimitiveTopology,
};

use super::Vertex;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DrawConfig<V: Vertex> {
    // FIXME  bad name
    //        does it means displacement?
    pub vertex: V,
    pub shader_config: Option<ShaderConfig>,
    pub texture_config: Option<TextureConfig>,
}

impl<V: Vertex> DrawConfig<V> {
    /// Apply changes to shader configuration in place.
    pub fn apply_shader_changes<F>(mut self, changes_fn: F) -> Self where
        F: FnOnce(ShaderConfig) -> ShaderConfig
    {
        self.shader_config = self.shader_config.map(changes_fn);

        self
    }
}
