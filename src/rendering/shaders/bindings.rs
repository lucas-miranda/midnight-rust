use wgpu::util::DeviceExt;

use crate::rendering::TextureView;

pub struct Bindings<'d> {
    device: &'d wgpu::Device,
    entries: Vec<BindingEntry>,
}

impl<'d> Bindings<'d> {
    pub(in crate::rendering) fn new(device: &'d wgpu::Device) -> Self {
        Self {
            device,
            entries: Default::default(),
        }
    }

    pub(in crate::rendering) fn push_texture_view(&mut self, texture_view: TextureView) {
        self.entries.push(BindingEntry::TextureView(texture_view.view));
        self.entries.push(BindingEntry::Sampler(self.device.create_sampler(&texture_view.sampler)));
    }

    pub fn push_uniforms<U>(&mut self, uniforms: &Vec<U>) where
        U: bytemuck::Pod + bytemuck::Zeroable
    {
        let uniform_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("uniforms buffer"),
            contents: bytemuck::cast_slice(uniforms.as_slice()),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        self.entries.push(BindingEntry::Buffer(uniform_buffer));
    }

    /*
    pub fn push_texture(&mut self, texture: &Texture) {
        self.push_texture_view(texture.view())
    }
    */

    pub fn collect<'a>(&'a self) -> Vec<wgpu::BindGroupEntry<'a>> {
        self.entries
            .iter()
            .enumerate()
            .map(|(i, e)| wgpu::BindGroupEntry {
                binding: i as u32,
                resource: e.resource(),
            })
            .collect()
    }
}

enum BindingEntry {
    Buffer(wgpu::Buffer),
    TextureView(wgpu::TextureView),
    Sampler(wgpu::Sampler),
}

impl BindingEntry {
    pub fn resource<'b>(&'b self) -> wgpu::BindingResource<'b> {
        match self {
            Self::Buffer(ref buf) => buf.as_entire_binding(),
            Self::TextureView(ref tex_view) => wgpu::BindingResource::TextureView(tex_view),
            Self::Sampler(ref sampler) => wgpu::BindingResource::Sampler(sampler),
        }
    }
}
