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
    PipelineLayoutFlags,
    CommandEncoderDescriptor, CommandEncoder, Queue, SurfaceCapabilities,
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

use super::{RenderBackendBuildError, RenderBackend, RenderPresentationSurface};

pub type Result<T> = std::result::Result<T, RenderBackendBuildError>;

pub struct RenderBackendBuilder<'a, A: Api> {
    phantom: PhantomData<A>,
    window: &'a Window,
    surface_size: (u32, u32),
}

impl<'a, A: Api> RenderBackendBuilder<'a, A> {
    pub(crate) fn new(window: &'a Window, surface_size: (u32, u32)) -> Self {
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
        let surface_caps = Self::retrieve_surface_capabilities(&adapter, &surface)?;
        println!("Surface caps: {:#?}", surface_caps);

        let OpenDevice { device, mut queue } = Self::open_logical_device(&adapter)?;

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

        unsafe { surface.configure(&device, &surface_config) }
            .map_err(RenderBackendBuildError::SurfaceConfigureFailed)?;

        let pipeline_layout = Self::create_pipeline_layout(&device)?;

        let vertex_shader = include_str!("shaders/p1.vert");
        let fragment_shader = include_str!("shaders/p1.frag");

        let pipeline = Self::create_pipeline(
            &device,
            &surface_config.format,
            &pipeline_layout,
            vertex_shader,
            fragment_shader,
        )?;

        let execution_context = Self::create_execution_context(&device, &mut queue)?;

        let device = Rc::new(device);
        let weak_device = Rc::downgrade(&device);

        Ok(RenderBackend::new(
            instance,
            device,
            queue,
            pipeline_layout,
            pipeline,
            execution_context,
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

        unsafe { A::Instance::init(&instance_desc) }
            .map_err(RenderBackendBuildError::InstanceFailed)
    }

    fn create_surface(instance: &A::Instance, window: &Window) -> Result<A::Surface> {
        unsafe { instance.create_surface(&window.internal_window()) }
            .map_err(RenderBackendBuildError::SurfaceFailed)
    }

    fn find_adapter(instance: &A::Instance) -> Result<(A::Adapter, Capabilities)> {
        let exposed = unsafe {
            let mut adapters = instance.enumerate_adapters();

            if adapters.is_empty() {
                return Err(RenderBackendBuildError::AdapterNotFound);
            }

            adapters.swap_remove(0)
        };

        Ok((exposed.adapter, exposed.capabilities))
    }

    fn retrieve_surface_capabilities(
        adapter: &A::Adapter,
        surface: &A::Surface,
    ) -> Result<SurfaceCapabilities> {
        unsafe { adapter.surface_capabilities(&surface) }
            .ok_or(RenderBackendBuildError::PresentationNotSupported)
    }

    fn open_logical_device(adapter: &A::Adapter) -> Result<OpenDevice<A>> {
        unsafe { adapter.open(Features::empty(), &Limits::default()) }
            .map_err(RenderBackendBuildError::LogicalDeviceOpenFailed)
    }

    fn create_pipeline_layout(device: &A::Device) -> Result<A::PipelineLayout> {
        let pipeline_layout_desc = PipelineLayoutDescriptor {
            label: None,
            flags: PipelineLayoutFlags::empty(),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        };

        unsafe { device.create_pipeline_layout(&pipeline_layout_desc) }
            .map_err(RenderBackendBuildError::PipelineLayoutFailed)
    }

    fn create_pipeline(
        device: &A::Device,
        format: &TextureFormat,
        pipeline_layout: &A::PipelineLayout,
        vertex_shader: &str,
        fragment_shader: &str,
    ) -> Result<A::RenderPipeline> {
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

        unsafe { device.create_render_pipeline(&pipeline_desc) }
            .map_err(RenderBackendBuildError::PipelineFailed)
    }

    fn create_execution_context(
        device: &A::Device,
        queue: &mut A::Queue,
    ) -> Result<ExecutionContext<A>> {
        let cmd_encoder_desc = CommandEncoderDescriptor {
            label: None,
            queue,
        };

        let mut cmd_encoder = unsafe { device.create_command_encoder(&cmd_encoder_desc) }
            .map_err(RenderBackendBuildError::CommandEncoderFailed)?;

        unsafe { cmd_encoder.begin_encoding(Some("init")) }
            .map_err(RenderBackendBuildError::CommandEncoderFailed)?;

        let init_fence_value = 1;
        let fence = unsafe {
            let mut fence = device.create_fence()
                .map_err(RenderBackendBuildError::CommandEncoderFailed)?;

            let init_cmd = cmd_encoder.end_encoding()
                .map_err(RenderBackendBuildError::CommandEncoderFailed)?;

            queue.submit(&[&init_cmd], Some((&mut fence, init_fence_value)))
                .map_err(RenderBackendBuildError::CommandEncoderFailed)?;

            device.wait(&fence, init_fence_value, !0)
                .map_err(RenderBackendBuildError::CommandEncoderFailed)?;

            cmd_encoder.reset_all(iter::once(init_cmd));

            fence
        };

        Ok(ExecutionContext {
            encoder: cmd_encoder,
            fence,
            fence_value: init_fence_value + 1,
            used_views: Vec::new(),
            used_cmd_bufs: Vec::new(),
            frames_recorded: 0,
        })
    }
}
