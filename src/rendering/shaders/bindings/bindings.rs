use std::mem;
use wgpu::util::DeviceExt;

use crate::rendering::TextureView;
use super::{BindingsDescriptorEntry, BindingsError};

/// Bindings which will be applied to shader.
/// It's created with a descriptor, so it only expects values to be filled at correct places.
/// Think this as empty slots waiting to be filled.
pub struct Bindings<'d> {
    device: &'d wgpu::Device,
    entries: Vec<Option<BindingEntry>>,
    descriptor: Vec<BindingsDescriptorEntry>,
}

// TODO
// - let user choose which index value is defining, instead always choose first found

impl<'d> Bindings<'d> {
    /// Place provided uniforms at first found Uniform binding entry.
    pub fn uniforms<U>(&mut self, uniforms: &Vec<U>) -> Result<(), BindingsError> where
        U: bytemuck::Pod + bytemuck::Zeroable
    {
        let uniform_buffer;
        let contents = bytemuck::cast_slice(uniforms.as_slice());

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
            BindingEntry::Buffer(uniform_buffer),
        )?;

        Ok(())
    }

    pub(in crate::rendering) fn new(
        device: &'d wgpu::Device,
        descriptor: Vec<BindingsDescriptorEntry>
    ) -> Self {
        let mut entries = Vec::with_capacity(descriptor.len());

        for _ in 0..descriptor.len() {
            entries.push(None);
        }

        Self {
            device,
            entries,
            descriptor,
        }
    }

    /// Place provided `TextureView` at first found Texture and Sampler binding entry.
    pub(in crate::rendering) fn texture_view(
        &mut self,
        texture_view: TextureView
    ) -> Result<(), BindingsError> {
        self.replace_entry(
            &BindingsDescriptorEntry::Texture,
            BindingEntry::TextureView(texture_view.view),
        )?;

        self.replace_entry(
            &BindingsDescriptorEntry::Sampler,
            BindingEntry::Sampler(self.device.create_sampler(&texture_view.sampler)),
        )?;

        Ok(())
    }

    pub(in crate::rendering) fn collect<'a>(
        &'a self
    ) -> Result<Vec<wgpu::BindGroupEntry<'a>>, BindingsError> {
        for (i, e) in self.entries.iter().enumerate() {
            if e.is_none() {
                return Err(BindingsError::EmptyValue {
                    expecting: self.descriptor[i],
                    at_index: i,
                })
            }
        }

        Ok(self.entries
               .iter()
               .enumerate()
               .map(|(i, e)| wgpu::BindGroupEntry {
                   binding: i as u32,
                   resource: e.as_ref()
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
    ) -> Option<&mut Option<BindingEntry>> {
        let mut index = None;

        for (i, d) in self.descriptor.iter().enumerate() {
            if d == descriptor {
                index = Some(i);
                break;
            }
        };

        match index {
            Some(i) => self.entries.get_mut(i),
            None => None
        }
    }

    fn replace_entry(
        &mut self,
        descriptor: &BindingsDescriptorEntry,
        entry: BindingEntry,
    ) -> Result<(), BindingsError> {
        match self.find_entry(descriptor) {
            Some(e) => {
                *e = Some(entry);
                Ok(())
            },
            None => Err(BindingsError::NotFound { expecting: *descriptor }),
        }
    }
}

/// Stores a resource at binding entry.
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
