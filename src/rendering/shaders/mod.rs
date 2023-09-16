pub(crate) mod builder;
pub use builder::ShaderFormat;

#[macro_use]
mod macros;

mod bindings;
pub use bindings::*;

mod stage;
pub use stage::*;

mod shader;
pub use shader::*;

mod descriptor;
pub use descriptor::*;

mod descriptor_error;
pub use descriptor_error::*;

mod raw_data;
pub use raw_data::ShaderRawData;

mod vertex_attribute;
pub use vertex_attribute::VertexAttribute;

mod uniforms;
pub use uniforms::*;

pub use wgpu::VertexFormat as AttributeFormat;

// TODO
//  - make a wrapper at bytemuck::cast_slice to be used at uniforms_as_slice

/// An instance of a shader, containing bindings.
/// It's created through [`ShaderBuilder`] and user should keep it somewhere (to avoid drop it).
pub trait ShaderInstance : ShaderInfo {
    /// Create a [`ShaderInstance`], by providing a [`Shader`].
    /// It's only used internally when shader is built and there is no point to be used manually.
    fn new(shader: Shader) -> Self where Self: Sized;

    fn world_view_projection_uniforms(&self) -> Option<&dyn WorldViewProjectionUniforms>;
    fn mut_world_view_projection_uniforms(&mut self) -> Option<&mut dyn WorldViewProjectionUniforms>;

    /// Fill provided [`Bindings`] with relevant information and return it.
    /// Values provided must match the [`BindingsDescriptorEntry`] described at shader build.
    fn bindings<'b>(&'b self, bindings: Bindings<'b>) -> Result<Bindings<'b>, BindingsError>;
}

/// Info about a shader.
pub trait ShaderInfo {
    /// Returns an opaque object representating a shader unique identifier.
    fn identifier(&self) -> Shader;
}

