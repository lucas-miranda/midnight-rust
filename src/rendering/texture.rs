pub use wgpu::TextureFormat;

use std::{
    hash::Hash,
    fmt::Display,
    path::Path,
    fs::File,
    io::{
        self,
        BufReader,
    }, num::NonZeroU32,
};

use wgpu::util::DeviceExt;
use image::io::Reader as ImageReader;
use crate::util::Size;
use super::{GraphicAdapter, backend::RenderBackend};

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
        adapter: &GraphicAdapter,
        format: TextureFormat,
        size: Size<u32>,
        data: &[u8]
    ) -> Self {

        /*
        let texture = adapter.backend().device.create_texture(&descriptor);

        adapter.backend().queue.write_texture(
            texture.as_image_copy(),
            data,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(NonZeroU32::new(
                    size.width * (format.describe().block_size as u32)
                ).unwrap()),
                rows_per_image: None,
            },
            extent
        );
        */

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

    pub fn load<P: AsRef<Path>>(adapter: &GraphicAdapter, path: P) -> io::Result<Self> {
        let contents = ImageReader::open(path)?
            // TODO  add a dedicated error to avoid unwrap this
            .decode().unwrap();

        let size = Size::new(contents.width(), contents.height());
        // TODO  add a dedicated error to avoid unwrap this
        let data = contents.as_rgba8().unwrap().as_raw();

        Ok(Self::new(adapter, TextureFormat::Bgra8UnormSrgb, size, data))
    }

    pub(super) fn id(&self) -> &TextureId {
        &self.id
    }

    pub(super) fn view<'v>(&self, device: &wgpu::Device, queue: &wgpu::Queue) -> TextureView<'v> {
        let extent = wgpu::Extent3d {
            width: self.size.width,
            height: self.size.height,
            depth_or_array_layers: 1,
        };

        let descriptor = wgpu::TextureDescriptor {
            size: extent,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: self.format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            label: None,
            view_formats: &[],
        };

        println!("Creating texture ({:?}) with size {:?}", self.format, self.size);

        //  17 x 37 = 629 px
        //  629 x 4 = 2516 bytes
        // 2516 x 8 = 20128 bits
        let texture = device.create_texture_with_data(
            &queue,
            &descriptor,
            self.data.as_slice(),
        );

        TextureView {
            id: self.id,
            view: texture.create_view(&wgpu::TextureViewDescriptor {
                label: None,
                ..wgpu::TextureViewDescriptor::default()
            }),
            sampler: wgpu::SamplerDescriptor {
                label: None,
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Nearest,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::FilterMode::Nearest,
                ..wgpu::SamplerDescriptor::default()
            },
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
    pub id: TextureId,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::SamplerDescriptor<'a>,
}
