use std::num::{NonZeroU32, NonZeroU8};

use crate::rendering::{
    AddressMode,
    CompareFunction,
    FilterMode,
    SamplerBorderColor,
    TextureAspect,
};

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct TextureConfig {
    pub aspect: TextureAspect,
    pub base_mip_level: u32,
    pub mip_level_count: Option<NonZeroU32>,
    pub base_array_layer: u32,
    pub array_layer_count: Option<NonZeroU32>,
    pub sampler: TextureSamplerConfig,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TextureSamplerConfig {
    pub sample_count: u32,
    pub address_mode_u: AddressMode,
    pub address_mode_v: AddressMode,
    pub address_mode_w: AddressMode,
    pub mag_filter: FilterMode,
    pub min_filter: FilterMode,
    pub mipmap_filter: FilterMode,
    //pub lod_min_clamp: f32,
    //ub lod_max_clamp: f32,
    pub compare: Option<CompareFunction>,
    pub anisotropy_clamp: Option<NonZeroU8>,
    pub border_color: Option<SamplerBorderColor>,
}

impl Default for TextureSamplerConfig {
    fn default() -> Self {
        Self {
            sample_count: 1,
            address_mode_u: Default::default(),
            address_mode_v: Default::default(),
            address_mode_w: Default::default(),
            mag_filter: Default::default(),
            min_filter: Default::default(),
            mipmap_filter: Default::default(),
            //lod_min_clamp: 0.0,
            //lod_max_clamp: std::f32::MAX,
            compare: None,
            anisotropy_clamp: None,
            border_color: None,
        }
    }
}
