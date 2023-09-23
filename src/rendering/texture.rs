pub use wgpu::TextureFormat;

use std::{
    fmt::Display,
    hash::Hash,
    num::NonZeroU32,
    path::Path,
};

use wgpu::{util::DeviceExt, TextureViewDimension, TextureSampleType, FilterMode, SamplerBindingType};
use image::io::Reader as ImageReader;
use crate::util::Size;
use super::{GraphicAdapter, TextureError, TextureConfig};

static mut NEXT_ID: TextureId = TextureId(1);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub(crate) struct TextureId(u32);

impl TextureId {
    pub const NONE: Self = TextureId(0);

    pub(super) fn next(&mut self) {
        self.0 += 1;
    }
}

impl Display for TextureId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}

pub struct Texture {
    id: TextureId,
    //texture: wgpu::Texture,
    format: TextureFormat,
    size: Size<u32>,
    data: Vec<u8>,
}

impl Texture {
    pub fn new(
        _adapter: &GraphicAdapter,
        format: TextureFormat,
        size: Size<u32>,
        data: &[u8]
    ) -> Self {
        let id = unsafe {
            let id = NEXT_ID;
            NEXT_ID.next();

            id
        };

        Self {
            id,
            format,
            size,
            data: data.to_owned(),
        }
    }

    pub fn load<P: AsRef<Path> + std::marker::Copy>(
        adapter: &GraphicAdapter,
        path: P
    ) -> Result<Self, TextureError> {
        let mut contents = ImageReader::open(path)
            .map_err(|e| TextureError::Open(e))?
            .decode()
            .map_err(|_| TextureError::UnsupportedFormat(path.as_ref().to_owned()))?;

        let size = Size::new(contents.width(), contents.height());

        let data = {
            let rgba = contents.as_mut_rgba8()
                               .ok_or_else(|| TextureError::RepresentationConversion)?;

            // convert rgba -> bgra
            for p in rgba.pixels_mut() {
                let [ r, g, b, a ] = p.0;
                p.0 = [ b, g, r, a ];
            }

            rgba.as_raw()
        };

        Ok(Self::new(adapter, TextureFormat::Bgra8UnormSrgb, size, data))
    }

    pub(super) fn id(&self) -> &TextureId {
        &self.id
    }

    pub(super) fn view<'v>(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        config: TextureConfig
    ) -> TextureView<'v> {
        let extent = wgpu::Extent3d {
            width: self.size.width,
            height: self.size.height,
            depth_or_array_layers: 1,
        };

        let descriptor = wgpu::TextureDescriptor {
            size: extent,
            mip_level_count: config.mip_level_count
                                   .unwrap_or_else(|| unsafe {
                                       NonZeroU32::new_unchecked(1u32)
                                   }).into(),
            sample_count: config.sampler.sample_count,
            dimension: wgpu::TextureDimension::D2,
            format: self.format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            label: None,
            view_formats: &[],
        };

        let texture = device.create_texture_with_data(
            &queue,
            &descriptor,
            self.data.as_slice(),
        );

        let filterable = config.sampler.mag_filter != FilterMode::Nearest
                || config.sampler.min_filter != FilterMode::Nearest
                || config.sampler.mipmap_filter != FilterMode::Nearest;

        TextureView {
            //id: self.id,
            view: texture.create_view(&wgpu::TextureViewDescriptor {
                label: None,
                format: None,
                dimension: Some(TextureViewDimension::D2),
                aspect: config.aspect,
                base_mip_level: config.base_mip_level,
                mip_level_count: config.mip_level_count,
                base_array_layer: config.base_array_layer,
                array_layer_count: config.array_layer_count,
            }),
            sampler: wgpu::SamplerDescriptor {
                label: None,
                address_mode_u: config.sampler.address_mode_u,
                address_mode_v: config.sampler.address_mode_v,
                address_mode_w: config.sampler.address_mode_w,
                mag_filter: config.sampler.mag_filter,
                min_filter: config.sampler.min_filter,
                mipmap_filter: config.sampler.mipmap_filter,
                compare: config.sampler.compare,
                anisotropy_clamp: config.sampler.anisotropy_clamp,
                border_color: config.sampler.border_color,
                ..Default::default()
            },
            sample_count: config.sampler.sample_count,
            sample_type: TextureSampleType::Float {
                filterable,
            },
            sampler_binding_type: if filterable {
                SamplerBindingType::Filtering
            } else {
                SamplerBindingType::NonFiltering
            },
            view_dimension: TextureViewDimension::D2,
        }
    }

    pub fn size(&self) -> Size<u32> {
        self.size
    }

    pub fn width(&self) -> u32 {
        self.size.width
    }

    pub fn height(&self) -> u32 {
        self.size.height
    }
}

pub(super) struct TextureView<'a> {
    //pub id: TextureId,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::SamplerDescriptor<'a>,
    pub sample_count: u32,
    pub sample_type: TextureSampleType,
    pub sampler_binding_type: SamplerBindingType,
    pub view_dimension: TextureViewDimension,
}
