use std::mem;

pub enum BindingsDescriptorEntry<U> {
    Uniform(std::marker::PhantomData<U>),
    Sampler,
    Texture,
}

impl<U> BindingsDescriptorEntry<U> {
    pub(in crate::rendering::shaders) fn layout_entry(
        &self,
        binding: u32,
    ) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
            count: None,
            ty: match self {
                Self::Uniform(_) => wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: wgpu::BufferSize::new(
                        mem::size_of::<U>() as _,
                    ),
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
