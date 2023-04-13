use super::AttributeFormat;

/// Describes a vertex attribute in a `Shader`.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VertexAttribute {
    /// Data format expected.
    pub format: AttributeFormat,

    /// Begin position (in bytes) at vertex buffer layout.
    pub offset: u64,

    /// Location index at shader.
    pub location: u32,
}

impl From<VertexAttribute> for wgpu::VertexAttribute {
    fn from(vertex_attr: VertexAttribute) -> Self {
        wgpu::VertexAttribute {
            format: vertex_attr.format,
            offset: vertex_attr.offset,
            shader_location: vertex_attr.location,
        }
    }
}

impl From<&VertexAttribute> for wgpu::VertexAttribute {
    fn from(vertex_attr: &VertexAttribute) -> Self {
        wgpu::VertexAttribute {
            format: vertex_attr.format,
            offset: vertex_attr.offset,
            shader_location: vertex_attr.location,
        }
    }
}

impl From<&mut VertexAttribute> for wgpu::VertexAttribute {
    fn from(vertex_attr: &mut VertexAttribute) -> Self {
        wgpu::VertexAttribute {
            format: vertex_attr.format,
            offset: vertex_attr.offset,
            shader_location: vertex_attr.location,
        }
    }
}
