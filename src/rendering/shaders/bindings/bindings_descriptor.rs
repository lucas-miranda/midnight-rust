use std::mem;

/// Describes a binding entry.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BindingsDescriptorEntry {
    Uniform { size: u64 },
    Sampler,
    Texture,
}

impl BindingsDescriptorEntry {
    pub fn uniform<U>() -> Self {
        Self::Uniform {
            size: mem::size_of::<U>() as _,
        }
    }

    pub fn sampler() -> Self {
        Self::Sampler
    }

    pub fn texture() -> Self {
        Self::Texture
    }

    pub(in crate::rendering::shaders) fn layout_entry(
        &self,
        binding: u32,
    ) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
            count: None,
            ty: match self {
                Self::Uniform { size } => wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: wgpu::BufferSize::new(*size),
                },
                Self::Sampler => wgpu::BindingType::Sampler(wgpu::SamplerBindingType::NonFiltering),
                Self::Texture => wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: false },
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                },
            }
        }
    }
}