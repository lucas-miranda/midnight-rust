use std::{
    collections::HashMap,
    iter,
    mem,
    ops::Deref,
};

use crate::rendering::{
    shaders::{
        Shader,
        VertexAttribute,
    },
    ShaderConfig
};

pub struct ShaderContext {
    pub vertex_module: wgpu::ShaderModule,
    pub fragment_module: wgpu::ShaderModule,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pipeline_layout: wgpu::PipelineLayout,
    pipeline: HashMap<ShaderConfig, ShaderPipeline>,
    surface_format: wgpu::TextureFormat,
    vertex_attributes: Vec<Vec<wgpu::VertexAttribute>>,
}

impl ShaderContext {
    pub(super) fn new<D, U>(
        shader: &Shader,
        device: D,
        surface_format: wgpu::TextureFormat,
        vertex_attributes: Vec<VertexAttribute>,
    ) -> Self where
        D: AsRef<wgpu::Device>
    {
        let vertex_attributes = iter::once(&vertex_attributes)
            .map(|attrs| attrs
                .into_iter()
                .map(wgpu::VertexAttribute::from)
                .collect::<Vec<wgpu::VertexAttribute>>()
            )
            .collect::<Vec<Vec<_>>>();

        //

        let vertex_module = device.as_ref().create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("shader label"),
            source: shader.vertex().data().into(),
        });

        let fragment_module = device.as_ref().create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("shader label"),
            source: shader.fragment().data().into(),
        });

        // -> Create pipeline layout

        // bind group layouts
        let bind_group_layout = {
            let bind_group_layout_desc = wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: wgpu::BufferSize::new(
                                mem::size_of::<U>() as _,
                            ),
                        },
                        count: None,
                    }
                ],
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

        Self {
            vertex_module,
            fragment_module,
            pipeline_layout,
            pipeline: Default::default(),
            bind_group_layout,
            surface_format,
            vertex_attributes,
        }
    }

    pub(in crate::rendering) fn pipeline<'p>(
        &'p mut self,
        device: &wgpu::Device,
        config: &ShaderConfig
    ) -> &'p ShaderPipeline {
        match self.pipeline.contains_key(config) {
            true => {
                println!("Using pipeline...");
                self.pipeline.get(config).unwrap()
            },
            false => {
                println!("Creating pipeline...");

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
