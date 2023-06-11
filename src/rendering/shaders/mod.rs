pub(crate) mod builder;
pub use builder::ShaderFormat;

#[macro_use]
mod macros;

mod bindings;
pub use bindings::*;

mod bindings_descriptor;
pub use bindings_descriptor::*;

mod shader_stage;
pub use shader_stage::*;

mod stage_kind;
pub use stage_kind::ShaderStageKind;

mod shader;
pub use shader::*;

mod descriptor;
pub use descriptor::*;

mod vertex_attribute;
pub use vertex_attribute::VertexAttribute;

pub use wgpu::VertexFormat as AttributeFormat;

pub trait ShaderUniformInstance : ShaderInstance {
    type Uniforms: bytemuck::Zeroable + bytemuck::Pod + bytemuck::NoUninit;

    fn uniforms(&self) -> &Self::Uniforms;
}

pub trait ShaderVertexInstance<V> : ShaderInstance where
    V: bytemuck::Pod + bytemuck::Zeroable
{
}

pub trait ShaderInstance : ShaderInfo {
    fn new(shader: Shader) -> Self where Self: Sized;
    fn uniforms_as_slice<'s>(&'s self) -> &'s [u8];
    fn bindings<'b>(&'b self, bindings: Bindings<'b>) -> Bindings<'b>;
}

pub trait ShaderInfo {
    fn id(&self) -> ShaderId;
}
