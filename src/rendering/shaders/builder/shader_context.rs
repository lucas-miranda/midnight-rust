use std::mem;

use wgpu_hal::{
    Api,
    Device,
    ShaderInput,
};

use crate::rendering::shaders::{Shader, ShaderData};

pub struct ShaderContext<A: Api> {
    pub vertex_module: A::ShaderModule,
    pub fragment_module: A::ShaderModule,
    pub pipeline_layout: A::PipelineLayout,
    pub pipeline: A::RenderPipeline,
}

impl<A: Api> ShaderContext<A> {
    pub fn new<T>(shader: &Shader, device: T, texture_format: wgpu_types::TextureFormat) -> Self where
        T: AsRef<A::Device>
    {
        let descriptor = wgpu_hal::ShaderModuleDescriptor {
            label: None,
            runtime_checks: false,
        };

        let vertex_module =
            match shader.vertex().data() {
                ShaderData::SpirV(spirv) => unsafe {
                    device.as_ref()
                        .create_shader_module(&descriptor, ShaderInput::SpirV(spirv))
                },
                #[cfg(feature = "shader-naga")]
                ShaderData::Naga(naga_shader) => {
                    unimplemented!();
                },
            }.unwrap();

        let fragment_module =
            match shader.fragment().data() {
                ShaderData::SpirV(spirv) => unsafe {
                    device.as_ref()
                        .create_shader_module(&descriptor, ShaderInput::SpirV(spirv))
                },
                #[cfg(feature = "shader-naga")]
                ShaderData::Naga(naga_shader) => {
                    unimplemented!();
                },
            }.unwrap();

        // create pipeline layout
        let pipeline_layout_desc = wgpu_hal::PipelineLayoutDescriptor {
            label: None,
            flags: wgpu_hal::PipelineLayoutFlags::empty(),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        };

        let pipeline_layout = unsafe {
                device.as_ref().create_pipeline_layout(&pipeline_layout_desc)
            }
            .unwrap();
            //.map_err(RenderBackendBuildError::PipelineLayoutFailed)

        // create pipeline
        let vertex_buffer_layout = wgpu_hal::VertexBufferLayout {
            //array_stride: mem::size_of::<super::Vertex>() as _,
            array_stride: mem::size_of::<crate::math::Vec2<f32>>() as _,
            step_mode: wgpu_types::VertexStepMode::Vertex,
            attributes: &[
                wgpu_types::VertexAttribute {
                    format: wgpu_types::VertexFormat::Float32x2,
                    offset: 0,
                    shader_location: 0,
                }
            ],
        };

        let pipeline_desc = wgpu_hal::RenderPipelineDescriptor {
            label: None,
            layout: &pipeline_layout,
            vertex_stage: wgpu_hal::ProgrammableStage {
                module: &vertex_module,
                entry_point: "main",
            },
            vertex_buffers: &[vertex_buffer_layout],
            fragment_stage: Some(wgpu_hal::ProgrammableStage {
                module: &fragment_module,
                entry_point: "main",
            }),
            primitive: wgpu_types::PrimitiveState {
                topology: wgpu_types::PrimitiveTopology::TriangleList,
                ..wgpu_types::PrimitiveState::default()
            },
            depth_stencil: None,
            multisample: wgpu_types::MultisampleState::default(),
            color_targets: &[wgpu_types::ColorTargetState {
                format: texture_format,
                blend: Some(wgpu_types::BlendState::ALPHA_BLENDING),
                write_mask: wgpu_types::ColorWrites::default(),
            }],
            multiview: None,
        };

        let pipeline = unsafe { device.as_ref().create_render_pipeline(&pipeline_desc) }
            .unwrap();
            //.map_err(RenderBackendBuildError::PipelineFailed)

        Self {
            vertex_module,
            fragment_module,
            pipeline_layout,
            pipeline,
        }
    }
}
