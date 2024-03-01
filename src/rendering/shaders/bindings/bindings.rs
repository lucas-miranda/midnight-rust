use std::mem;
use wgpu::util::DeviceExt;

use crate::rendering::TextureView;
use super::{
    BindingKind,
    BindingsDescriptorEntry,
    BindingsError,
};

/// Bindings which will be applied to shader.
/// It's created with a descriptor, so it only expects values to be filled at correct places.
/// Think this as empty slots waiting to be filled.
pub struct Bindings<'d> {
    device: &'d wgpu::Device,
    entries: Vec<BindingEntry>,
}

// TODO
// - let user choose which index value is defining, instead always choose first found

impl<'d> Bindings<'d> {
    /// Place provided uniforms at first found Uniform binding entry.
    pub fn uniforms<U>(&mut self, uniforms: &[U]) -> Result<(), BindingsError> where
        U: bytemuck::Pod + bytemuck::Zeroable
    {
        let uniform_buffer;
        //let c = vec![uniforms];
        let contents = bytemuck::cast_slice(uniforms);

        let misaligned_bytes = wgpu::util::align_to(contents.len(), super::UNIFORM_BINDING_ALIGNMENT) - contents.len();
        if misaligned_bytes > 0 {
            // misaligned contents
            let aligned_contents: Vec<_>
                = contents.iter()
                          .map(|n| *n)
                          .chain((0..misaligned_bytes).map(|_| 0u8))
                          .collect();

            uniform_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("uniforms buffer"),
                contents: aligned_contents.as_slice(),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });
        } else {
            // aligned contents
            uniform_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("uniforms buffer"),
                contents,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });
        }

        self.replace_entry(
            &BindingsDescriptorEntry::Uniform { size: wgpu::util::align_to(mem::size_of::<U>() as _, 16) },
            RawBinding::Buffer(uniform_buffer),
        )?;

        Ok(())
    }

    pub(in crate::rendering) fn new(
        device: &'d wgpu::Device,
        descriptor: Vec<BindingsDescriptorEntry>
    ) -> Self {
        Self {
            device,
            entries: descriptor.iter()
                  .map(|e| BindingEntry::new(*e))
                  .collect(),
        }
    }

    /// Place provided `TextureView` at first found Texture and Sampler binding entry.
    pub(in crate::rendering) fn texture_view(
        &mut self,
        texture_view: TextureView
    ) -> Result<(), BindingsError> {
        self.replace_entry(
            &BindingsDescriptorEntry::Texture {
                sample_type: texture_view.sample_type,
                multisampled: texture_view.sample_count > 1,
                view_dimension: texture_view.view_dimension,
            },
            RawBinding::TextureView(texture_view.view),
        )?;

        self.replace_entry(
            &BindingsDescriptorEntry::Sampler(texture_view.sampler_binding_type),
            RawBinding::Sampler(self.device.create_sampler(&texture_view.sampler)),
        )?;

        Ok(())
    }

    pub(in crate::rendering) fn collect<'a>(
        &'a self
    ) -> Result<Vec<wgpu::BindGroupEntry<'a>>, BindingsError> {
        for (i, e) in self.entries.iter().enumerate() {
            if !e.has_value() {
                return Err(BindingsError::EmptyValue {
                    expecting: e.descriptor,
                    at_index: i,
                })
            }
        }

        Ok(self.entries
               .iter()
               .enumerate()
               .map(|(i, e)| wgpu::BindGroupEntry {
                   binding: i as u32,
                   resource: e.raw
                              .as_ref()
                              // NOTE  safe to unwrap, it was checked already before
                              .unwrap()
                              .resource(),
               })
               .collect()
        )
    }

    fn find_entry(
        &mut self,
        descriptor: &BindingsDescriptorEntry,
    ) -> Option<&mut BindingEntry> {
        for e in self.entries.iter_mut() {
            if e.descriptor.match_descriptor(descriptor) {
                return Some(e);
            }
        };

        None
    }

    fn replace_entry(
        &mut self,
        descriptor: &BindingsDescriptorEntry,
        raw: RawBinding,
    ) -> Result<(), BindingsError> {
        match self.find_entry(descriptor) {
            Some(e) => {
                (*e).raw = Some(raw);
                Ok(())
            },
            None => Err(BindingsError::NotFound { expecting: *descriptor }),
        }
    }
}

//

struct BindingEntry {
    pub kind: BindingKind,
    pub descriptor: BindingsDescriptorEntry,
    pub raw: Option<RawBinding>,
}

impl BindingEntry {
    pub fn new(descriptor: BindingsDescriptorEntry) -> Self {
        Self {
            kind: descriptor.kind(),
            descriptor,
            raw: None,
        }
    }

    pub fn has_value(&self) -> bool {
        self.raw.is_some()
    }
}

//

/// Stores a resource at binding entry.
enum RawBinding {
    Buffer(wgpu::Buffer),
    TextureView(wgpu::TextureView),
    Sampler(wgpu::Sampler),
}

impl RawBinding {
    pub fn resource<'b>(&'b self) -> wgpu::BindingResource<'b> {
        match self {
            Self::Buffer(ref buf) => buf.as_entire_binding(),
            Self::TextureView(ref tex_view) => wgpu::BindingResource::TextureView(tex_view),
            Self::Sampler(ref sampler) => wgpu::BindingResource::Sampler(sampler),
        }
    }
}
