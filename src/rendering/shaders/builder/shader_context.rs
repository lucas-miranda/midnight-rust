use std::{
    collections::HashMap,
    ops::Deref,
};

use crate::rendering::{
    shaders::{
        BindingsDescriptorEntry,
        VertexAttribute,
        ShaderDescriptor,
        ShaderStageKind, ShaderDescriptorError,
    },
    ShaderConfig,
};

use super::ShaderProcessor;

pub struct ShaderContext {
    pub vertex_module: wgpu::ShaderModule,
    pub fragment_module: wgpu::ShaderModule,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub reuse_pipeline: bool,
    pipeline_layout: wgpu::PipelineLayout,
    pipeline: HashMap<ShaderConfig, ShaderPipeline>,
    surface_format: wgpu::TextureFormat,
    vertex_attributes: Vec<Vec<wgpu::VertexAttribute>>,
    bindings: Vec<BindingsDescriptorEntry>,
}

impl ShaderContext {
    pub(super) fn new<D>(
        processor: ShaderProcessor,
        descriptor: &ShaderDescriptor,
        device: D,
        surface_format: wgpu::TextureFormat,
        vertex_attributes: Vec<VertexAttribute>,
        bindings: Vec<BindingsDescriptorEntry>,
    ) -> Result<Self, ShaderDescriptorError> where
        D: AsRef<wgpu::Device>
    {
        let vertex_attributes = vec![
            vertex_attributes
                .into_iter()
                .map(wgpu::VertexAttribute::from)
                .collect(),
        ];

        //

        let vertex_module = {
            let vertex_stage = descriptor.process_stage(&ShaderStageKind::Vertex, &processor)?;

            device.as_ref().create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: vertex_stage.data().into(),
            })
        };

        let fragment_module = {
            let frag_stage = descriptor.process_stage(&ShaderStageKind::Fragment, &processor)?;

            device.as_ref().create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: frag_stage.data().into(),
            })
        };

        // -> Create pipeline layout

        // bind group layouts
        let bind_group_layout = {
            let entries: Vec<_> = bindings
                .iter()
                .enumerate()
                .map(|(i, e)| e.layout_entry(i as u32))
                .collect();

            let bind_group_layout_desc = wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: entries.as_slice(),
            };

            device.as_ref().create_bind_group_layout(&bind_group_layout_desc)
        };

        let pipeline_layout = device
            .as_ref()
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
        });

        Ok(Self {
            vertex_module,
            fragment_module,
            bind_group_layout,
            reuse_pipeline: true,
            pipeline_layout,
            pipeline: Default::default(),
            surface_format,
            vertex_attributes,
            bindings,
        })
    }

    pub(in crate::rendering) fn bindings_descriptor(&self) -> Vec<BindingsDescriptorEntry> {
        self.bindings.clone()
    }

    pub(in crate::rendering) fn pipeline<'p>(
        &'p mut self,
        device: &wgpu::Device,
        config: &ShaderConfig
    ) -> &'p ShaderPipeline {
        match self.reuse_pipeline && self.pipeline.contains_key(config) {
            true => {
                //println!("Using pipeline...");
                // NOTE  safe to unwrap  key was checked before
                self.pipeline.get(config).unwrap()
            },
            false => {
                //println!("Creating pipeline...");

                let buffers: Vec<_> = self.vertex_attributes
                    .iter()
                    .map(|attributes| wgpu::VertexBufferLayout {
                        array_stride: attributes
                            .iter()
                            .fold(0u64, |s, attr| s + attr.format.size()),
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes,
                    })
                    .collect::<Vec<wgpu::VertexBufferLayout>>();

                let handle = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: None,
                    layout: Some(&self.pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &self.vertex_module,
                        entry_point: "main",
                        buffers: &buffers,
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &self.fragment_module,
                        entry_point: "main",
                        targets: &[Some(self.surface_format.into())]
                    }),
                    primitive: *config.primitive_state(),
                    depth_stencil: None,
                    multisample: wgpu::MultisampleState::default(),
                    multiview: None,
                });

                self.pipeline.insert(
                    *config,
                    ShaderPipeline {
                        handle,
                    }
                );

                // NOTE  safe to unwrap  key was inserted before
                self.pipeline.get(config).unwrap()
            },
        }
    }
}

pub struct ShaderPipeline {
    pub handle: wgpu::RenderPipeline,
}

impl Deref for ShaderPipeline {
    type Target = wgpu::RenderPipeline;

    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}
