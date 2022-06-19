use std::{
    borrow::Borrow,
    iter,
    mem::ManuallyDrop,
    rc::Rc,
};

use wgpu_hal::{
    Api,
    FenceValue,
    Instance,
    Device,
    Surface,
    CommandEncoder,
    TextureBarrier,
    TextureUses,
    TextureViewDescriptor, RenderPassDescriptor, ColorAttachment,
    Attachment, AttachmentOps, Queue, CommandEncoderDescriptor,
    BufferBarrier,
    SurfaceError,
    DeviceError,
};

use wgpu_types::{
    ImageSubresourceRange,
    TextureViewDimension,
    Extent3d,
    Color,
};

/*
use gfx_hal::{
    command::{
        ClearColor, ClearValue, CommandBuffer, CommandBufferFlags, SubpassContents,
        RenderAttachmentInfo,
    },
    device::Device,
    image::Extent,
    pool::CommandPool,
    queue::{Queue, QueueGroup},
    pso::{Rect, Viewport},
    window::PresentationSurface,
    Backend,
};
*/

use super::{
    RenderBackendOperationError,
    RenderPresentationSurface,
};

const RENDER_TIMEOUT_NS: u64 = 1_000_000_000;
const ACQUIRE_TIMEOUT_NS: u64 = 1_000_000_000;

pub type Result<T> = std::result::Result<T, RenderBackendOperationError>;

pub struct ExecutionContext<A: Api> {
    pub encoder: A::CommandEncoder,
    pub fence: A::Fence,
    pub fence_value: FenceValue,
    pub used_views: Vec<A::TextureView>,
    pub used_cmd_bufs: Vec<A::CommandBuffer>,
    pub frames_recorded: usize,
}

impl<A: Api> ExecutionContext<A> {
    unsafe fn wait_and_clear(&mut self, device: &A::Device) {
        device.wait(&self.fence, self.fence_value, !0).unwrap();
        self.encoder.reset_all(self.used_cmd_bufs.drain(..));

        for view in self.used_views.drain(..) {
            device.destroy_texture_view(view);
        }

        self.frames_recorded = 0;
    }
}

pub struct RenderContext<A: Api> {
    pub execution_context_index: usize,
    pub surface_texture: A::SurfaceTexture,
    pub surface_texture_view: A::TextureView,
}

pub struct RenderBackend<A: Api> {
    instance: A::Instance,
    device: Rc<A::Device>,
    queue: A::Queue,
    pipeline_layout: A::PipelineLayout,
    pipeline: A::RenderPipeline,
    contexts: Vec<ExecutionContext<A>>,
    context_index: usize,
    presentation_surface: RenderPresentationSurface<A>,
}

impl<A: Api> RenderBackend<A> {
    pub(super) fn new(
        instance: A::Instance,
        device: Rc<A::Device>,
        queue: A::Queue,
        pipeline_layout: A::PipelineLayout,
        pipeline: A::RenderPipeline,
        execution_context: ExecutionContext<A>,
        presentation_surface: RenderPresentationSurface<A>,
    ) -> Self {
        Self {
            instance,
            device,
            queue,
            pipeline_layout,
            pipeline,
            contexts: vec![execution_context],
            context_index: 0,
            presentation_surface,
        }
    }

    pub fn presentation_surface(&self) -> &RenderPresentationSurface<A> {
        &self.presentation_surface
    }

    pub fn mut_presentation_surface(&mut self) -> &mut RenderPresentationSurface<A> {
        &mut self.presentation_surface
    }

    pub fn render(&mut self) -> Result<()> {
        let render_context = self.prepare_render()?;

        // draw
        {
            let context = &mut self.contexts[self.context_index];

            unsafe {
                /*
                context.encoder
                    .set_bind_group(&self.pipeline_layout, 0, &self.global_group, &[]);
                */
                context.encoder.draw(0, 3, 0, 1);
            }
        }

        self.submit(render_context)?;
        Ok(())
    }

    fn prepare_render(&mut self) -> Result<RenderContext<A>> {
        // reconfigure if needed
        self.presentation_surface.reconfigure_swapchain(false);

        let context = &mut self.contexts[self.context_index];
        let surface_texture = unsafe { self.presentation_surface.mut_surface().acquire_texture(None) }
            .unwrap()
            .unwrap()
            .texture;

        let target_barrier0 = TextureBarrier {
            texture: surface_texture.borrow(),
            range: ImageSubresourceRange::default(),
            usage: TextureUses::UNINITIALIZED..TextureUses::COLOR_TARGET,
        };

        unsafe {
            context.encoder.begin_encoding(Some("frame")).unwrap();
            context.encoder.transition_textures(iter::once(target_barrier0));
        }

        let surface_view_desc = TextureViewDescriptor {
            label: None,
            format: *self.presentation_surface.surface_format(),
            dimension: TextureViewDimension::D2,
            usage: TextureUses::COLOR_TARGET,
            range: ImageSubresourceRange::default(),
        };

        let surface_texture_view = unsafe {
            self.device
                .create_texture_view(surface_texture.borrow(), &surface_view_desc)
                .unwrap()
        };

        let pass_desc = RenderPassDescriptor {
            label: None,
            extent: Extent3d {
                width: self.presentation_surface.width(),
                height: self.presentation_surface.height(),
                depth_or_array_layers: 1,
            },
            sample_count: 1,
            color_attachments: &[ColorAttachment {
                target: Attachment {
                    view: &surface_texture_view,
                    usage: TextureUses::COLOR_TARGET,
                },
                resolve_target: None,
                ops: AttachmentOps::STORE,
                clear_value: Color {
                    r: 0.1,
                    g: 0.2,
                    b: 0.3,
                    a: 1.0,
                }
            }],
            depth_stencil_attachment: None,
            multiview: None,
        };

        unsafe {
            context.encoder.begin_render_pass(&pass_desc);
            context.encoder.set_render_pipeline(&self.pipeline);
        }

        Ok(RenderContext {
            execution_context_index: self.context_index,
            surface_texture,
            surface_texture_view,
        })
    }

    fn submit(&mut self, render_context: RenderContext<A>) -> Result<()> {
        let RenderContext {
            execution_context_index,
            surface_texture,
            surface_texture_view,
        } = render_context;

        let context = &mut self.contexts[execution_context_index];
        context.frames_recorded += 1;
        let do_fence = context.frames_recorded > 100;

        let target_barrier1 = TextureBarrier {
            texture: surface_texture.borrow(),
            range: ImageSubresourceRange::default(),
            usage: TextureUses::COLOR_TARGET..TextureUses::PRESENT,
        };

        unsafe {
            context.encoder.end_render_pass();
            context.encoder.transition_textures(iter::once(target_barrier1));
        }

        unsafe {
            let cmd_buf = context.encoder.end_encoding().unwrap();

            let fence_param = if do_fence {
                Some((&mut context.fence, context.fence_value))
            } else {
                None
            };

            self.queue.submit(&[&cmd_buf], fence_param).unwrap();

            // TODO  maybe we should just discard everything when present failed?
            //       instead of continuing as nothing happened
            if let Err(present_err) = self.queue.present(self.presentation_surface.mut_surface(), surface_texture) {
                match present_err {
                    SurfaceError::Device(device_err) => {
                        match device_err {
                            DeviceError::OutOfMemory => panic!("{}", device_err),
                            _ => (),
                        }
                    },
                    SurfaceError::Other(other_err) => panic!("{}", other_err),
                    _ => (),
                }
            }

            context.used_cmd_bufs.push(cmd_buf);
            context.used_views.push(surface_texture_view);
        }

        if do_fence {
            println!("Context switch from {}", self.context_index);
            let old_fence_value = context.fence_value;

            if self.contexts.len() == 1 {
                let cmd_encoder_desc = CommandEncoderDescriptor {
                    label: None,
                    queue: &self.queue,
                };

                self.contexts.push(unsafe {
                    ExecutionContext {
                        encoder: self.device.create_command_encoder(&cmd_encoder_desc)
                                     .unwrap(),
                        fence: self.device.create_fence().unwrap(),
                        fence_value: 0,
                        used_views: Vec::new(),
                        used_cmd_bufs: Vec::new(),
                        frames_recorded: 0,
                    }
                });
            }

            self.context_index = (self.context_index + 1) % self.contexts.len();
            let next = &mut self.contexts[self.context_index];

            unsafe {
                next.wait_and_clear(&self.device);
            }

            next.fence_value = old_fence_value + 1;
        }

        Ok(())
    }
}

impl<A: Api> Drop for RenderBackend<A> {
    fn drop(&mut self) {
        /*
        let device = ManuallyDrop::into_inner(self.device);
        let instance = ManuallyDrop::into_inner(self.instance);
        let render_passes = ManuallyDrop::into_inner(self.render_passes);
        let pipeline_layouts = ManuallyDrop::into_inner(self.pipeline_layouts);
        let pipelines = ManuallyDrop::into_inner(self.pipelines);
        let command_pool = ManuallyDrop::into_inner(self.command_pool);

        let submission_complete_fence = ManuallyDrop::into_inner(
            self.submission_complete_fence
        );

        let rendering_complete_semaphore = ManuallyDrop::into_inner(
            self.rendering_complete_semaphore
        );

        let presentation_surface = ManuallyDrop::into_inner(self.presentation_surface);

        unsafe {
            for pipeline in pipelines {
                self.device.destroy_graphics_pipeline(pipeline);
            }

            for pipeline_layout in pipeline_layouts {
                self.device.destroy_pipeline_layout(pipeline_layout);
            }

            for render_pass in render_passes {
                self.device.destroy_render_pass(render_pass);
            }

            presentation_surface.destroy_surface(&instance);
        }
        */
    }
}
