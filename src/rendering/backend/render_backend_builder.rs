use std::{
   iter,
    rc::Rc, marker::PhantomData,
};

use wgpu_hal::{
    Adapter,
    Api,
    Capabilities,
    InstanceDescriptor,
    InstanceFlags,
    Instance,
    OpenDevice,
    SurfaceConfiguration,
    CompositeAlphaMode,
    TextureUses,
    Surface,
    Device,
    RenderPipelineDescriptor,
    ProgrammableStage,
    ShaderModuleDescriptor,
    ShaderInput,
    PipelineLayoutDescriptor,
    BindGroupLayoutFlags,
    PipelineLayoutFlags,
    CommandEncoderDescriptor, CommandEncoder, Queue,
};

use wgpu_types::{
    Features,
    Limits,
    PresentMode,
    TextureFormat,
    Extent3d,
    PrimitiveState,
    PrimitiveTopology,
    MultisampleState,
    ColorTargetState,
    BlendState,
    ColorWrites,
};

/*
use gfx_hal::{
    adapter::{Adapter, PhysicalDevice},
    command::Level,
    device::Device,
    format::{ChannelType, Format},
    image::Layout,
    pass::{
        Attachment, AttachmentLoadOp, AttachmentOps, AttachmentStoreOp, Subpass, SubpassDesc,
    },
    pool::{CommandPool, CommandPoolCreateFlags},
    pso::{
        BlendState, ColorBlendDesc, ColorMask, EntryPoint, Face, GraphicsPipelineDesc,
        InputAssemblerDesc, Primitive, PrimitiveAssemblerDesc, Rasterizer, Specialization,
    },
    queue::{QueueFamily, QueueGroup},
    window::Surface,
    Backend,
    Instance,
};
*/

use crate::{
    rendering::{
        backend::ExecutionContext,
        shaders::{
            builder::{
                ShaderBuilder,
                ShaderFormat,
            },
            ShaderData,
        },
    },
    window::Window,
};

use super::{RenderBackendInitError, RenderBackend, RenderPresentationSurface};

pub type Result<T> = std::result::Result<T, RenderBackendInitError>;

/*
type HALCommandPool<B> = <B as Backend>::CommandPool;
type HALCommandBuffer<B> = <B as Backend>::CommandBuffer;
type HALDevice<B> = <B as Backend>::Device;
type HALFence<B> = <B as Backend>::Fence;
type HALInstance<B> = <B as Backend>::Instance;
type HALPipelineLayout<B> = <B as Backend>::PipelineLayout;
type HALRenderPass<B> = <B as Backend>::RenderPass;
type HALSemaphore<B> = <B as Backend>::Semaphore;
type HALSurface<B> = <B as Backend>::Surface;
*/

pub struct RenderBackendBuilder<'a, A: Api> {
    phantom: PhantomData<A>,
    window: &'a Window,
    surface_size: (u32, u32),
}

impl<'a, A: Api> RenderBackendBuilder<'a, A> {
    pub(super) fn new(window: &'a Window, surface_size: (u32, u32)) -> Self {
        Self {
            phantom: Default::default(),
            window,
            surface_size,
        }
    }

    pub fn build(self) -> Result<RenderBackend<A>> {
        let app_name = "app name";
        let app_version = 1;

        let instance = Self::create_instance(app_name)?;
        let mut surface = Self::create_surface(&instance, self.window)?;
        let (adapter, capabilities) = Self::find_adapter(&instance)?;

        let surface_caps = unsafe {
            adapter.surface_capabilities(&surface)
        }.unwrap(); //.ok_or(HALInitError::SurfaceCapabilitiesFailed)?;

        println!("Surface caps: {:#?}", surface_caps);

        let OpenDevice { device, mut queue } = Self::create_logical_device(&adapter)?;

        let surface_config = SurfaceConfiguration {
            swap_chain_size: 3
                .max(*surface_caps.swap_chain_sizes.start())
                .min(*surface_caps.swap_chain_sizes.end()),
            present_mode: PresentMode::Fifo,
            composite_alpha_mode: CompositeAlphaMode::Opaque,
            format: TextureFormat::Bgra8UnormSrgb,
            extent: Extent3d {
                width: self.surface_size.0,
                height: self.surface_size.1,
                depth_or_array_layers: 1,
            },
            usage: TextureUses::COLOR_TARGET,
        };

        unsafe {
            surface.configure(&device, &surface_config)
        }.unwrap(); //.map_err(HALInitError::SurfaceConfigureFailed)?;

        let pipeline_layout = Self::create_pipeline_layout(&device)?;

        let vertex_shader = include_str!("shaders/p1.vert");
        let fragment_shader = include_str!("shaders/p1.frag");

        let pipeline = Self::create_pipeline(
            &device,
            &surface_config.format,
            &pipeline_layout,
            vertex_shader,
            fragment_shader,
        );

        let cmd_encoder_desc = CommandEncoderDescriptor {
            label: None,
            queue: &queue,
        };

        let mut cmd_encoder = unsafe { device.create_command_encoder(&cmd_encoder_desc) }.unwrap();

        unsafe {
            cmd_encoder.begin_encoding(Some("init"))
        }.unwrap();

        let init_fence_value = 1;
        let fence = unsafe {
            let mut fence = device.create_fence().unwrap();
            let init_cmd = cmd_encoder.end_encoding().unwrap();

            queue.submit(&[&init_cmd], Some((&mut fence, init_fence_value)))
                 .unwrap();

            device.wait(&fence, init_fence_value, !0).unwrap();
            cmd_encoder.reset_all(iter::once(init_cmd));

            fence
        };

        let device = Rc::new(device);
        let weak_device = Rc::downgrade(&device);

        Ok(RenderBackend::new(
            instance,
            device,
            queue,
            pipeline_layout,
            pipeline,
            ExecutionContext {
                encoder: cmd_encoder,
                fence,
                fence_value: init_fence_value + 1,
                used_views: Vec::new(),
                used_cmd_bufs: Vec::new(),
                frames_recorded: 0,
            },
            RenderPresentationSurface::new(
                weak_device,
                adapter,
                surface,
                surface_config.format,
                self.surface_size.0,
                self.surface_size.1,
            )
        ))
    }

    fn create_instance(name: &str) -> Result<A::Instance> {
        let instance_desc = InstanceDescriptor {
            name,
            flags: if cfg!(debug_assertions) {
                InstanceFlags::all()
            } else {
                InstanceFlags::empty()
            },
        };

        Ok(unsafe {
            A::Instance::init(&instance_desc)
        }.unwrap()) //.map_err(HALInitError::UnsupportedBackend)
    }

    fn create_surface(instance: &A::Instance, window: &Window)
        -> Result<A::Surface>
    {
        Ok(unsafe {
            instance.create_surface(&window.internal_window())
        }.unwrap()) //.map_err(HALInitError::SurfaceCreationFailed)
    }

    fn find_adapter(instance: &A::Instance) -> Result<(A::Adapter, Capabilities)> {
        let exposed = unsafe {
            let mut adapters = instance.enumerate_adapters();

            if adapters.is_empty() {
                //return Err(HALInitError::AdapterNotFound);
                panic!("adapter not found");
            }

            adapters.swap_remove(0)
        };

        Ok((exposed.adapter, exposed.capabilities))
    }

    fn create_logical_device(adapter: &A::Adapter)
        -> Result<OpenDevice<A>>
    {
        Ok(unsafe {
            adapter.open(Features::empty(), &Limits::default())
        }.unwrap()) //.map_err(HALInitError::LogicalDeviceCreationFailed)
    }

    /*
    fn create_command_pool(device: &HALDevice<B>, queue_group: &QueueGroup<B>)
        -> Result<(HALCommandPool<B>, HALCommandBuffer<B>)>
    {
        unsafe {
            let mut command_pool = device
                .create_command_pool(queue_group.family, CommandPoolCreateFlags::empty())
                .map_err(HALInitError::DeviceOutOfMemory)?;

            Ok((command_pool, command_pool.allocate_one(Level::Primary)))
        }
    }

    fn get_surface_color_format(surface: &HALSurface<B>, adapter: &Adapter<B>) -> Format {
        match surface.supported_formats(&adapter.physical_device) {
            Some(formats) => {
                formats
                    .into_iter()
                    .find(|format| format.base_format().1 == ChannelType::Srgb)
                    .unwrap_or_else(|| {
                        // default format
                        *formats.get(0).unwrap_or(&Format::Rgba8Srgb)
                    })
            },
            None => Format::Rgba8Srgb,
        }
    }

    fn create_render_pass(device: &HALDevice<B>, color_format: &Format)
        -> Result<HALRenderPass<B>>
    {
        let color_attachment = Attachment {
            format: Some(*color_format),
            samples: 1,
            ops: AttachmentOps::new(AttachmentLoadOp::Clear, AttachmentStoreOp::Store),
            stencil_ops: AttachmentOps::DONT_CARE,
            layouts: Layout::Undefined..Layout::Present,
        };

        let subpass = SubpassDesc {
            colors: &[(0, Layout::ColorAttachmentOptimal)],
            depth_stencil: None,
            inputs: &[],
            resolves: &[],
            preserves: &[],
        };

        let render_pass = unsafe {
            device.create_render_pass(
                    iter::once(color_attachment), iter::once(subpass), iter::empty()
                )
                .map_err(HALInitError::DeviceOutOfMemory)
        };

        render_pass
    }
    */

    fn create_pipeline_layout(device: &A::Device) -> Result<A::PipelineLayout> {
        let pipeline_layout_desc = PipelineLayoutDescriptor {
            label: None,
            flags: PipelineLayoutFlags::empty(),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        };

        Ok(unsafe {
            device.create_pipeline_layout(&pipeline_layout_desc)
                .unwrap()
                //.map_err(HALInitError::DeviceOutOfMemory)
        })
    }

    fn create_pipeline(
        device: &A::Device,
        format: &TextureFormat,
        pipeline_layout: &A::PipelineLayout,
        vertex_shader: &str,
        fragment_shader: &str,
    ) -> A::RenderPipeline
    {
        let shader_builder = ShaderBuilder::default();

        let shader = shader_builder.build(
            ShaderFormat::GLSL,
            vertex_shader,
            fragment_shader,
        );

        let shader_desc = ShaderModuleDescriptor {
            label: None,
            runtime_checks: false,
        };

        let vertex_shader_module = match shader.vertex() {
            Some(stage) => {
                match stage.data() {
                    ShaderData::SpirV(spirv) => Some(unsafe {
                        device.create_shader_module(&shader_desc, ShaderInput::SpirV(spirv))
                              .expect("Failed to create vertex shader module")
                    }),
                    #[cfg(feature = "shader-naga")]
                    ShaderData::Naga(naga_shader) => {
                        unimplemented!();
                    },
                    _ => panic!("Shader data format not supported."),
                }
            },
            None => None,
        }.unwrap();

        let fragment_shader_module = match shader.fragment() {
            Some(stage) => {
                match stage.data() {
                    ShaderData::SpirV(spirv) => Some(unsafe {
                        device.create_shader_module(&shader_desc, ShaderInput::SpirV(spirv))
                              .expect("Failed to create fragment shader module")
                    }),
                    #[cfg(feature = "shader-naga")]
                    ShaderData::Naga(naga_shader) => {
                        unimplemented!();
                    },
                    _ => panic!("Shader data format not supported."),
                }
            },
            None => None,
        }.unwrap();

        let pipeline_desc = RenderPipelineDescriptor {
            label: None,
            layout: pipeline_layout,
            vertex_stage: ProgrammableStage {
                module: &vertex_shader_module,
                entry_point: "main",
            },
            vertex_buffers: &[],
            fragment_stage: Some(ProgrammableStage {
                module: &fragment_shader_module,
                entry_point: "main",
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                ..PrimitiveState::default()
            },
            depth_stencil: None,
            multisample: MultisampleState::default(),
            color_targets: &[ColorTargetState {
                format: *format,
                blend: Some(BlendState::ALPHA_BLENDING),
                write_mask: ColorWrites::default(),
            }],
            multiview: None,
        };

        unsafe {
            device.create_render_pipeline(&pipeline_desc)
        }.unwrap()




        /*
        let shader_builder = ShaderBuilder::default();

        let shader = shader_builder.build(
            ShaderFormat::GLSL,
            vertex_shader,
            fragment_shader,
        );

        let vertex_shader_module = match shader.vertex() {
            Some(stage) => {
                match stage.data() {
                    ShaderData::SpirV(spirv) => Some(unsafe {
                        device.create_shader_module(&spirv)
                              .expect("Failed to create vertex shader module")
                    }),
                    #[cfg(feature = "shader-naga")]
                    ShaderData::Naga(naga_shader) => {
                        unimplemented!();
                    },
                    _ => panic!("Shader data format not supported."),
                }
            },
            None => None,
        }.unwrap();

        let fragment_shader_module = match shader.fragment() {
            Some(stage) => {
                match stage.data() {
                    ShaderData::SpirV(spirv) => Some(unsafe {
                        device.create_shader_module(&spirv)
                              .expect("Failed to create fragment shader module")
                    }),
                    #[cfg(feature = "shader-naga")]
                    ShaderData::Naga(naga_shader) => {
                        unimplemented!();
                    },
                    _ => panic!("Shader data format not supported."),
                }
            },
            None => None,
        }.unwrap();

        let (vs_entry, fs_entry) = (
            EntryPoint {
                entry: "main",
                module: &vertex_shader_module,
                specialization: Specialization::EMPTY,
            },
            EntryPoint {
                entry: "main",
                module: &fragment_shader_module,
                specialization: Specialization::EMPTY,
            },
        );

        let primitive_assembler = PrimitiveAssemblerDesc::Vertex {
            buffers: &[],
            attributes: &[],
            input_assembler: InputAssemblerDesc::new(Primitive::TriangleList),
            vertex: vs_entry,
            tessellation: None,
            geometry: None,
        };

        let mut pipeline_desc = GraphicsPipelineDesc::new(
            primitive_assembler,
            Rasterizer {
                cull_face: Face::NONE,
                ..Rasterizer::FILL
            },
            Some(fs_entry),
            pipeline_layout,
            Subpass {
                index: 0,
                main_pass: render_pass,
            },
        );

        pipeline_desc.blender.targets.push(ColorBlendDesc {
            mask: ColorMask::ALL,
            blend: Some(BlendState::ALPHA),
        });

        let pipeline = unsafe {
            device.create_graphics_pipeline(&pipeline_desc, None)
                  .expect("Failed to create graphics pipeline")
        };

        unsafe {
            device.destroy_shader_module(vertex_shader_module);
            device.destroy_shader_module(fragment_shader_module);
        };

        pipeline
        */
    }
}
