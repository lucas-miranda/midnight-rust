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
    pub vertex: V,
    pub shader_config: Option<ShaderConfig>,
    pub texture_config: Option<TextureConfig>,
}
