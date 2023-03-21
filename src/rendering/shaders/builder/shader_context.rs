use wgpu_hal::{
    Api,
    Device,
    ShaderError,
    ShaderInput,
    VertexBufferLayout,
};

use crate::rendering::shaders::{Shader, ShaderData};

pub struct ShaderContext<A: Api> {
    pub vertex_module: A::ShaderModule,
    pub fragment_module: A::ShaderModule,
    pub pipeline_layout: A::PipelineLayout,
    pub pipeline: A::RenderPipeline,
}

impl<A: Api> ShaderContext<A> {
    pub(super) fn new<T>(
        shader: &Shader,
        device: T,
        texture_format: wgpu_types::TextureFormat,
        vertex_buffers: &[Vec<wgpu_types::VertexAttribute>],
    ) -> Self where
        T: AsRef<A::Device>
    {
        let descriptor = wgpu_hal::ShaderModuleDescriptor {
            label: Some("shader label"),
            runtime_checks: true,
        };

        let vertex_module = Self::create_module(
            device.as_ref(),
            &descriptor,
            shader.vertex().data(),
        )
        .unwrap();

        let fragment_module = Self::create_module(
            device.as_ref(),
            &descriptor,
            shader.fragment().data(),
        )
        .unwrap();

        // prepare vertex buffers

        // map wgpu_types::VertexAttribute -> wgpu_hal::VertexBufferLayout
        let vertex_buffers = vertex_buffers
                .iter()
                .map(|attributes| wgpu_hal::VertexBufferLayout {
                    array_stride: attributes
                        .iter()
                        .fold(0u64, |s, attr| s + attr.format.size()),
                    step_mode: wgpu_types::VertexStepMode::Vertex,
                    attributes,
                })
                .collect::<Vec<VertexBufferLayout>>();

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
        let pipeline_desc = wgpu_hal::RenderPipelineDescriptor {
            label: None,
            layout: &pipeline_layout,
            vertex_stage: wgpu_hal::ProgrammableStage {
                module: &vertex_module,
                entry_point: "main",
            },
            //vertex_buffers: &[vertex_buffer_layout],
            vertex_buffers: &vertex_buffers,
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
            color_targets: &[Some(wgpu_types::ColorTargetState {
                format: texture_format,
                blend: Some(wgpu_types::BlendState::ALPHA_BLENDING),
                write_mask: wgpu_types::ColorWrites::default(),
            })],
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

    fn create_module(
        device: &A::Device,
        descriptor: &wgpu_hal::ShaderModuleDescriptor,
        shader_data: &ShaderData,
    ) -> Result<A::ShaderModule, ShaderError> {
        match shader_data {
            ShaderData::SpirV(spirv) => unsafe {
                device.create_shader_module(&descriptor, ShaderInput::SpirV(spirv))
            },
            #[cfg(feature = "shader-naga")]
            ShaderData::Naga(naga_shader) => {
                unimplemented!();
            },
        }
    }
}
