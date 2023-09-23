use std::mem;

use wgpu::FilterMode;

use crate::rendering::{
    DrawConfig,
    SamplerBindingType,
    TextureSampleType,
    TextureViewDimension,
    Vertex,
};

use super::{BindingKind, BindingsError};

/// Describes a binding entry.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BindingsDescriptorEntry {
    Uniform { size: u64 },
    Sampler(SamplerBindingType),
    Texture {
        sample_type: TextureSampleType,
        multisampled: bool,
        view_dimension: TextureViewDimension,
    },
}

impl BindingsDescriptorEntry {
    pub fn uniform<U>() -> Self {
        Self::Uniform {
            size: wgpu::util::align_to(mem::size_of::<U>(), super::UNIFORM_BINDING_ALIGNMENT) as _,
        }
    }

    pub fn sampler(binding_type: SamplerBindingType) -> Self {
        Self::Sampler(binding_type)
    }

    pub fn texture(
        sample_type: TextureSampleType,
        multisampled: bool,
        view_dimension: TextureViewDimension
    ) -> Self {
        Self::Texture {
            sample_type,
            multisampled,
            view_dimension,
        }
    }

    pub fn kind(&self) -> BindingKind {
        match self {
            Self::Uniform { .. } => BindingKind::Uniform,
            Self::Sampler(_) => BindingKind::Sampler,
            Self::Texture { .. } => BindingKind::Texture,
        }
    }

    pub(super) fn match_descriptor(&self, bindings_descriptor: &BindingsDescriptorEntry) -> bool {
        match self {
            Self::Sampler(binding_type) => {
                if let BindingsDescriptorEntry::Sampler(other_binding_type) = bindings_descriptor {
                    match binding_type {
                        SamplerBindingType::Filtering => *other_binding_type != SamplerBindingType::Comparison,
                        _ => other_binding_type == binding_type,
                    }
                } else {
                    false
                }
            }
            Self::Texture { sample_type, multisampled, view_dimension } => {
                if let BindingsDescriptorEntry::Texture {
                    sample_type: other_sample_type,
                    multisampled: other_multisampled,
                    view_dimension: other_view_dimension
                } = bindings_descriptor {
                    if !multisampled && *other_multisampled {
                        // not multisampled restrict other to be not multisampled also
                        return false;
                    }

                    if let TextureSampleType::Float { filterable } = sample_type {
                        if let TextureSampleType::Float { filterable: other_filterable } = other_sample_type {
                            if !filterable && *other_filterable {
                                // not filterable restrict other to be not filterable also
                                return false;
                            }
                        }
                    } else if other_sample_type != sample_type {
                        return false
                    }

                    return other_view_dimension == view_dimension;
                }

                false
            },
            _ => self == bindings_descriptor
        }
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
                Self::Sampler(ty) => wgpu::BindingType::Sampler(*ty),
                Self::Texture{ sample_type, multisampled, view_dimension } =>
                    wgpu::BindingType::Texture {
                        sample_type: *sample_type,
                        multisampled: *multisampled,
                        view_dimension: *view_dimension,
                    },
            }
        }
    }

    pub(in crate::rendering) fn validate_config<V>(&self, config: &DrawConfig<V>) -> Result<(), BindingsError> where
        V: Vertex,
    {
        match self {
            Self::Uniform { .. } => Ok(()),
            Self::Sampler(_) => Ok(()),
            Self::Texture { sample_type, multisampled, .. } => {
                let c = config.texture_config.unwrap_or_default();

                if let TextureSampleType::Float { filterable } = sample_type {
                    if !*filterable {
                        if c.sampler.mag_filter != FilterMode::Nearest {
                            return Err(BindingsError::ValidationFailed(format!(
                                "Sample type isn't filterable, mag filter '{:?}' isn't supported",
                                c.sampler.mag_filter
                            )))
                        }

                        if c.sampler.min_filter != FilterMode::Nearest {
                            return Err(BindingsError::ValidationFailed(format!(
                                "Sample type isn't filterable, min filter '{:?}' isn't supported",
                                c.sampler.min_filter
                            )))
                        }

                        if c.sampler.mipmap_filter != FilterMode::Nearest {
                            return Err(BindingsError::ValidationFailed(format!(
                                "Sample type isn't filterable, mipmap filter '{:?}' isn't supported",
                                c.sampler.mipmap_filter
                            )))
                        }
                    }
                }

                if *multisampled {
                    if c.sampler.sample_count <= 1 {
                        return Err(BindingsError::ValidationFailed(
                            "Multisampled needs 1 or more sample count at TextureConfig".to_owned()
                        ))
                    }
                } else {
                    if c.sampler.sample_count > 1 {
                        return Err(BindingsError::ValidationFailed(
                            "Not multisampled must have only 1 at sample count at TextureConfig".to_owned()
                        ))
                    }
                }

                Ok(())
            },
        }
    }
}
