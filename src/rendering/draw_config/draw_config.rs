pub use wgpu::{
    Face,
    FrontFace,
    IndexFormat,
    PolygonMode,
    PrimitiveState,
    PrimitiveTopology,
};

use super::{
    shaders::{
        Shader,
        ShaderInfo,
    },
    Vertex,
};


/*
impl<V: Vertex> DrawConfig<V> {
    pub const EMPTY: Self = DrawConfig {
        vertex: V::default(),
        shader_config: None,
    };
}
*/




