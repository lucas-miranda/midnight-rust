pub(crate) mod builder;

#[macro_use]
mod macros;

mod shader_stage;
pub use shader_stage::*;

mod shader;
pub use shader::*;

mod vertex_attribute;
pub use vertex_attribute::VertexAttribute;

pub use wgpu::VertexFormat as AttributeFormat;

pub trait ShaderInstance {
    type Uniforms: bytemuck::Zeroable + bytemuck::Pod + bytemuck::NoUninit;

    fn id(&self) -> ShaderId;
    fn uniforms(&self) -> &Self::Uniforms;
}
