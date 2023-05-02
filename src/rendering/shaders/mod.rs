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

pub trait ShaderUniformInstance: ShaderInstance {
    type Uniforms: bytemuck::Zeroable + bytemuck::Pod + bytemuck::NoUninit;

    fn uniforms(&self) -> &Self::Uniforms;
}

pub trait ShaderInstance {
    fn new(shader: Shader) -> Self where Self: Sized;
    fn id(&self) -> ShaderId;
    fn uniforms_as_slice<'s>(&'s self) -> &'s [u8];
}
