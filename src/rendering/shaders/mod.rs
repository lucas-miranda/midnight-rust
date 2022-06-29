pub(crate) mod builder;

#[macro_use]
mod macros;

mod shader_stage;
pub use shader_stage::{ShaderData, ShaderStage};

mod shader;
pub use shader::{Shader, ShaderId};

mod vertex_attribute;
pub use vertex_attribute::VertexAttribute;

pub use wgpu_types::VertexFormat as AttributeFormat;
