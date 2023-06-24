pub(crate) mod builder;
pub use builder::ShaderFormat;

#[macro_use]
mod macros;

mod bindings;
pub use bindings::*;

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

// TODO
//  - make a wrapper at bytemuck::cast_slice to be used at uniforms_as_slice

/// An instance of a shader, containing bindings.
/// It's created through [`ShaderBuilder`] and user should keep it somewhere (to avoid drop it).
pub trait ShaderInstance : ShaderInfo {
    /// Create a [`ShaderInstance`], by providing a [`Shader`].
    /// It's only used internally when shader is built and there is no point to be used manually.
    fn new(shader: Shader) -> Self where Self: Sized;

    /// Returns uniforms as a slice, using something like [`bytemuck::cast_slice`] to do the job.
    /// If there is no uniforms, a empty slice is enough.
    fn uniforms_as_slice<'s>(&'s self) -> &'s [u8];

    /// Fill provided [`Bindings`] with relevant information and return it.
    /// Values provided must match the [`BindingsDescriptorEntry`] described at shader build.
    fn bindings<'b>(&'b self, bindings: Bindings<'b>) -> Bindings<'b>;
}

/// Info about a shader.
pub trait ShaderInfo {
    /// Returns an opaque object representating a shader unique identifier.
    fn identifier(&self) -> Shader;
}
