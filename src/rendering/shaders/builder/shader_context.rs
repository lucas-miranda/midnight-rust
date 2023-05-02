use std::mem;

use crate::rendering::shaders::Shader;

pub struct ShaderContext {
    pub vertex_module: wgpu::ShaderModule,
    pub fragment_module: wgpu::ShaderModule,
    pub pipeline_layout: wgpu::PipelineLayout,
    pub pipeline: wgpu::RenderPipeline,
    pub bind_group_layout: wgpu::BindGroupLayout,
}

impl ShaderContext {
    pub(super) fn new<D, U>(
        shader: &Shader,
        device: D,
        surface_format: wgpu::TextureFormat,
        vertex_buffers: &[Vec<wgpu::VertexAttribute>],
        primitive_topology: super::PrimitiveTopology,
    ) -> Self where
        D: AsRef<wgpu::Device>
    {
        let vertex_module = device.as_ref().create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("shader label"),
            source: shader.vertex().data().into(),
        });

        let fragment_module = device.as_ref().create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("shader label"),
            source: shader.fragment().data().into(),
        });

        // -> Prepare vertex buffers

        // map our VertexAttribute -> wgpu_hal::VertexBufferLayout
        let vertex_buffers = vertex_buffers
                .iter()
                .map(|attributes| wgpu::VertexBufferLayout {
                    array_stride: attributes
                        .iter()
                        .fold(0u64, |s, attr| s + attr.format.size()),
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes,
                })
                .collect::<Vec<wgpu::VertexBufferLayout>>();

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

        //

        let pipeline_layout = device.as_ref()
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            });

        // -> Create pipeline
        let pipeline = device.as_ref().create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vertex_module,
                entry_point: "main",
                buffers: &vertex_buffers,
            },
            fragment: Some(wgpu::FragmentState {
                module: &fragment_module,
                entry_point: "main",
                targets: &[Some(surface_format.into())]
            }),
            primitive: wgpu::PrimitiveState {
                topology: primitive_topology,
                ..wgpu::PrimitiveState::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        Self {
            vertex_module,
            fragment_module,
            pipeline_layout,
            pipeline,
            bind_group_layout,
        }
    }
}
