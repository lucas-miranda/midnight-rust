use std::{
    iter,
    mem::ManuallyDrop,
    rc::Rc,
};

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

use crate::{
    rendering::shaders::{
        builder::{
            ShaderBuilder,
            ShaderFormat,
        },
        ShaderData,
    },
    window::Window,
};

use super::{
    HALInitError,
    RenderPipeline,
    RenderSurface, RenderPipelineOperationError,
};

pub type Result<T> = std::result::Result<T, HALInitError>;

type HALCommandPool<B> = <B as Backend>::CommandPool;
type HALCommandBuffer<B> = <B as Backend>::CommandBuffer;
type HALDevice<B> = <B as Backend>::Device;
type HALFence<B> = <B as Backend>::Fence;
type HALInstance<B> = <B as Backend>::Instance;
type HALPipelineLayout<B> = <B as Backend>::PipelineLayout;
type HALRenderPass<B> = <B as Backend>::RenderPass;
type HALSemaphore<B> = <B as Backend>::Semaphore;
type HALSurface<B> = <B as Backend>::Surface;

/// Hardware abstraction layer interface which can be accessed through `GraphicBackend`
pub struct HAL<B: Backend> {
    pub(super) device: ManuallyDrop<Rc<B::Device>>,
    //pub(super) queue_group: QueueGroup<B>,
    //pub(super) adapter: Adapter<B>,
    //pub(super) instance: ManuallyDrop<B::Instance>,
    //pub(super) surface: ManuallyDrop<B::Surface>,
    //pub(super) render_passes: ManuallyDrop<Vec<B::RenderPass>>,
    //pub(super) pipeline_layouts: ManuallyDrop<Vec<B::PipelineLayout>>,
    //pub(super) pipelines: ManuallyDrop<Vec<B::GraphicsPipeline>>,
    //pub(super) command_pool: ManuallyDrop<B::CommandPool>,
    //pub(super) command_buffer: B::CommandBuffer,
    //pub(super) submission_complete_fence: ManuallyDrop<B::Fence>,
    //pub(super) rendering_complete_semaphore: ManuallyDrop<B::Semaphore>,

    //need_reconfigure_swapchain: bool,
    //requested_swapchain_size: Option<(u32, u32)>,
    //surface_extent: Extent2D,

    pub(super) render_pipeline: ManuallyDrop<RenderPipeline<B>>,
}

impl<B: Backend> HAL<B> {
    pub fn new(window: &Window, width: u32, height: u32) -> Result<Self> {
        let app_name = "app name";
        let app_version = 1;

        let instance = Self::create_instance(app_name, app_version)?;
        let surface = Self::create_surface(&instance, window)?;
        let adapter = Self::find_adapter(&instance, &surface)?;
        let (device, queue_group) = Self::create_logical_device(&surface, &adapter)?;
        let (command_pool, command_buf) = Self::create_command_pool(&device, &queue_group)?;
        let surface_color_format = Self::get_surface_color_format(&surface, &adapter);
        let render_pass = Self::create_render_pass(&device, &surface_color_format)?;
        let pipeline_layout = Self::create_pipeline_layout(&device)?;

        let vertex_shader = include_str!("shaders/p1.vert");
        let fragment_shader = include_str!("shaders/p1.frag");

        let pipeline = Self::create_pipeline(
            &device,
            &render_pass,
            &pipeline_layout,
            vertex_shader,
            fragment_shader,
        );

        let submission_complete_fence = device.create_fence(true).expect("Out of memory");
        let rendering_complete_semaphore = device.create_semaphore().expect("Out of memory");

        let device = Rc::new(device);
        let weak_device = Rc::downgrade(&device);

        Ok(Self {
            device: ManuallyDrop::new(device),
            queue_group,
            //adapter,
            instance: ManuallyDrop::new(instance),
            //surface: ManuallyDrop::new(surface),
            //render_passes: ManuallyDrop::new(vec![render_pass]),
            //pipeline_layouts: ManuallyDrop::new(vec![pipeline_layout]),
            //pipelines: ManuallyDrop::new(vec![pipeline]),
            command_pool: ManuallyDrop::new(command_pool),
            //command_buffer: command_buf,
            submission_complete_fence: ManuallyDrop::new(submission_complete_fence),
            rendering_complete_semaphore: ManuallyDrop::new(rendering_complete_semaphore),

            render_pipeline: ManuallyDrop::new(RenderPipeline::new(
                                weak_device.clone(),
                                render_pass,
                                pipeline_layout,
                                pipeline,
                                command_buf,
                             )),
            render_surface: ManuallyDrop::new(RenderSurface::new(
                                weak_device,
                                adapter,
                                surface,
                                width,
                                height,
                            )),
        })
    }

    pub fn render_surface(&self) -> &RenderSurface<B> {
        &self.render_surface
    }

    pub fn mut_render_surface(&mut self) -> &mut RenderSurface<B> {
        &mut self.render_surface
    }

    pub fn render(&mut self) {
        self.render_pipeline.wait_rendering(self);
        self.render_surface.reconfigure_swapchain(false);

        let frame_image = match self.render_pipeline.render(&mut self.render_surface) {
            Ok(frame_image) => frame_image,
            Err(e) => {
                if let RenderPipelineOperationError::AcquireImageFailed(_) = e {
                    self.render_surface.request_reconfigure_swapchain();
                    return;
                }

                panic!("{}", e);
            },
        };

        match self.render_pipeline.submit(self, frame_image) {
            Ok(_) => {
            },
            Err(e) => {
                if let RenderPipelineOperationError::FramePresentFailed(_) = e {
                    self.render_surface.request_reconfigure_swapchain();
                    return;
                }

                panic!("{}", e);
            }
        };
    }

    /*
    pub fn is_requested_configure_swapchain(&self) -> bool {
        self.need_reconfigure_swapchain
    }

    pub fn request_configure_swapchain(&mut self) {
        self.need_reconfigure_swapchain = true;
    }

    pub fn request_configure_swapchain_with(&mut self, width: u32, height: u32) {
        self.need_reconfigure_swapchain = true;
        self.requested_swapchain_size = Some((width, height));
    }

    pub(super) fn reconfigure_swapchain(&mut self) {
        if !self.need_reconfigure_swapchain {
            return;
        }

        use gfx_hal::window::SwapchainConfig;

        let caps = self.surface.capabilities(&self.adapter.physical_device);

        match self.requested_swapchain_size.take() {
            Some(size) => {
                self.surface_extent.width = size.0;
                self.surface_extent.height = size.1;
            },
            None => (),
        }

        let swapchain_config =
            SwapchainConfig::from_caps(&caps, self.surface_color_format, self.surface_extent);

        // macOS fullscreen slowdown fix
        /*
        if caps.image_count.contains(&3) {
            swapchain_config.image_count = 3;
        }
        */

        self.surface_extent = swapchain_config.extent;
        let device = self.device.upgrade().unwrap();

        unsafe {
            self.surface
               .configure_swapchain(&device, swapchain_config)
               .expect("Failed to configure swapchain");
        };

        self.need_reconfigure_swapchain = false;
    }
    */

    fn create_instance(name: &str, version: u32) -> Result<HALInstance<B>> {
        B::Instance::create(name, version)
            .map_err(HALInitError::UnsupportedBackend)
    }

    fn create_surface(instance: &HALInstance<B>, window: &Window)
        -> Result<HALSurface<B>>
    {
        unsafe {
            instance
                .create_surface(&window.internal_window())
                .map_err(HALInitError::SurfaceCreationFailed)
        }
    }

    fn find_adapter(instance: &HALInstance<B>, surface: &HALSurface<B>)
        -> Result<Adapter<B>>
    {
        instance
            .enumerate_adapters()
            .into_iter()
            .find(|a| a.queue_families
                .iter()
                .any(|qf| qf.queue_type().supports_graphics()
                    && surface.supports_queue_family(qf)
                )
            )
            .ok_or_else(|| HALInitError::AdapterNotFound)
    }

    fn create_logical_device(surface: &HALSurface<B>, adapter: &Adapter<B>)
        -> Result<(HALDevice<B>, QueueGroup<B>)>
    {
        let queue_family = adapter
            .queue_families
            .iter()
            .find(|family| surface.supports_queue_family(family)
                && family.queue_type().supports_graphics()
            )
            .ok_or_else(|| HALInitError::QueueFamilyNotFound)?;

        let mut gpu = unsafe {
            adapter
                .physical_device
                .open(&[(queue_family, &[1.0])], gfx_hal::Features::empty())
                .map_err(HALInitError::LogicalDeviceCreationFailed)
        }?;

        let queue_group_index = gpu
            .queue_groups
            .iter()
            .position(|qg| qg.family == queue_family.id());

        let queue_group = match queue_group_index {
            Some(index) => Ok(gpu.queue_groups.remove(index)),
            None => Err(HALInitError::CommandQueueGroupNotFound),
        }?;

        Ok((gpu.device, queue_group))
    }

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

    fn create_pipeline_layout(device: &HALDevice<B>) -> Result<HALPipelineLayout<B>> {
        unsafe {
            device.create_pipeline_layout(iter::empty(), iter::empty())
                .map_err(HALInitError::DeviceOutOfMemory)
        }
    }

    fn create_pipeline(
        device: &B::Device,
        render_pass: &B::RenderPass,
        pipeline_layout: &B::PipelineLayout,
        vertex_shader: &str,
        fragment_shader: &str,
    ) -> B::GraphicsPipeline
    {
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
    }
}

impl<B: Backend> Drop for HAL<B> {
    fn drop(&mut self) {
        let device = ManuallyDrop::into_inner(self.device);
        let instance = ManuallyDrop::into_inner(self.instance);
        //let render_passes = ManuallyDrop::into_inner(self.render_passes);
        //let pipeline_layouts = ManuallyDrop::into_inner(self.pipeline_layouts);
        //let pipelines = ManuallyDrop::into_inner(self.pipelines);
        let command_pool = ManuallyDrop::into_inner(self.command_pool);

        let submission_complete_fence = ManuallyDrop::into_inner(
            self.submission_complete_fence
        );

        let rendering_complete_semaphore = ManuallyDrop::into_inner(
            self.rendering_complete_semaphore
        );

        //let render_surface = ManuallyDrop::take(&mut self.render_surface);

        unsafe {
            device.destroy_semaphore(rendering_complete_semaphore);
            device.destroy_fence(submission_complete_fence);
        }

        let _ = ManuallyDrop::into_inner(self.render_pipeline);

        unsafe {
            //let device = ManuallyDrop::take(&mut self.device);
            //let instance = ManuallyDrop::take(&mut self.instance);
            //let surface = ManuallyDrop::take(&mut self.surface);

            //device.destroy_semaphore(rendering_complete_semaphore);
            //device.destroy_fence(submission_complete_fence);

            /*
            for pipeline in pipelines {
                device.destroy_graphics_pipeline(pipeline);
            }

            for pipeline_layout in pipeline_layouts {
                device.destroy_pipeline_layout(pipeline_layout);
            }

            for render_pass in render_passes {
                device.destroy_render_pass(render_pass);
            }
            */

            //let _ = ManuallyDrop::into_inner(self.render_pipeline);

            device.destroy_command_pool(command_pool);
            //surface.unconfigure_swapchain(&device);
            //instance.destroy_surface(surface);
        }

        let surface = {
            let render_surface = ManuallyDrop::into_inner(self.render_surface);
            render_surface.surface
        };

        unsafe {
            instance.destroy_surface(surface);
        }
    }
}
