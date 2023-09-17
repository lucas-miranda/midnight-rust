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
            size: wgpu::util::align_to(mem::size_of::<U>(), super::UNIFORM_BINDING_ALIGNMENT) as _,
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
                Self::Sampler => wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                Self::Texture => wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                },
            }
        }
    }
}
