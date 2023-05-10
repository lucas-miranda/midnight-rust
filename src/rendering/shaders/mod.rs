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

pub trait ShaderUniformInstance : ShaderInstance {
    type Uniforms: bytemuck::Zeroable + bytemuck::Pod + bytemuck::NoUninit;

    fn uniforms(&self) -> &Self::Uniforms;
}

pub trait ShaderInstance : ShaderInfo {
    fn new(shader: Shader) -> Self where Self: Sized;
    fn uniforms_as_slice<'s>(&'s self) -> &'s [u8];
}

pub trait ShaderInfo {
    fn id(&self) -> ShaderId;
}
