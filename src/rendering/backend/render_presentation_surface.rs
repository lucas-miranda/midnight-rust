use std::rc::Weak;

use wgpu_hal::{
    Adapter,
    Api,
    Surface,
    SurfaceCapabilities,
    SurfaceConfiguration,
    TextureUses,
};

use wgpu_types::{
    CompositeAlphaMode,
    TextureFormat,
    PresentMode,
    Extent3d,
};

use crate::math::Vector2;

/*
use gfx_hal::{
    adapter::Adapter,
    format::{ChannelType, Format},
    window::{Extent2D, PresentationSurface, Surface, SurfaceCapabilities, SwapchainConfig},
    Backend,
    Instance,
};
*/

pub struct RenderPresentationSurface<A: Api> {
    device: Weak<A::Device>,
    adapter: A::Adapter,
    surface: A::Surface,
    surface_format: TextureFormat,
    need_reconfigure_swapchain: bool,
    requested_swapchain_size: Option<(u32, u32)>,
    surface_extent: Vector2<u32>, // TODO change to Size<T>
}

impl<A: Api> RenderPresentationSurface<A> {
    pub(super) fn new(
        device: Weak<A::Device>,
        adapter: A::Adapter,
        surface: A::Surface,
        surface_format: TextureFormat,
        width: u32,
        height: u32,
    ) -> Self {
        //let surface_color_format = Self::get_surface_color_format(&adapter, &surface);

        Self {
            device,
            adapter,
            surface,
            surface_format,
            need_reconfigure_swapchain: true,
            requested_swapchain_size: None,
            surface_extent: Vector2::new(width, height),
            //surface_color_format,
        }
    }

    pub fn size(&self) -> (u32, u32) {
        (self.surface_extent.x, self.surface_extent.y)
    }

    pub fn width(&self) -> u32 {
        self.surface_extent.x
    }

    pub fn height(&self) -> u32 {
        self.surface_extent.y
    }

    pub fn has_request_reconfigure_swapchain(&self) -> bool {
        self.need_reconfigure_swapchain
    }

    pub fn request_reconfigure_swapchain(&mut self) {
        self.need_reconfigure_swapchain = true;
    }

    pub fn request_reconfigure_swapchain_with(&mut self, width: u32, height: u32) {
        self.need_reconfigure_swapchain = true;
        self.requested_swapchain_size = Some((width, height));
    }

    pub(super) fn adapter(&self) -> &A::Adapter {
        &self.adapter
    }

    pub(super) fn surface(&self) -> &A::Surface {
        &self.surface
    }

    pub(super) fn mut_surface(&mut self) -> &mut A::Surface {
        &mut self.surface
    }

    pub(super) fn surface_format(&self) -> &TextureFormat {
        &self.surface_format
    }

    pub(super) fn capabilities(&self) -> Option<SurfaceCapabilities> {
        //self.surface.capabilities(&self.adapter.physical_device)
        unsafe {
            self.adapter.surface_capabilities(&self.surface)
        } //.ok_or(HALInitError::SurfaceCapabilitiesFailed)?;
    }

    /*
    pub(super) fn swapchain_config(&self) -> SwapchainConfig {
        SwapchainConfig::from_caps(
            &self.capabilities(),
            self.surface_color_format,
            self.surface_extent,
        )
    }
    */

    pub(super) fn reconfigure_swapchain(&mut self, force: bool) {
        if !self.need_reconfigure_swapchain && !force {
            return;
        }

        match self.requested_swapchain_size.take() {
            Some(ref size) => self.surface_extent = size.into(),
            None => (),
        }

        let device = self.device.upgrade().unwrap();

        let surface_caps = self.capabilities().unwrap();
        let surface_config = SurfaceConfiguration {
            swap_chain_size: 3
                .max(*surface_caps.swap_chain_sizes.start())
                .min(*surface_caps.swap_chain_sizes.end()),
            present_mode: PresentMode::Fifo,
            composite_alpha_mode: CompositeAlphaMode::Opaque,
            format: TextureFormat::Bgra8UnormSrgb,
            extent: Extent3d {
                width: self.width(),
                height: self.height(),
                depth_or_array_layers: 1,
            },
            usage: TextureUses::COLOR_TARGET,
            view_formats: Vec::default(),
        };

        unsafe {
            self.surface.configure(&device, &surface_config)
        }.unwrap();

        println!("Surface reconfigured to {}", self.surface_extent);
        self.need_reconfigure_swapchain = false;

        /*
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
        //if caps.image_count.contains(&3) {
            //swapchain_config.image_count = 3;
        //}

        self.surface_extent = swapchain_config.extent;

        let device = self.device.upgrade().unwrap();

        unsafe {
            self.surface
               .configure_swapchain(&device, swapchain_config)
               .expect("Failed to configure swapchain");
        };

        self.need_reconfigure_swapchain = false;
        */
    }

    /*
    pub(super) unsafe fn destroy_surface(&mut self, instance: &B::Instance) {
        instance.destroy_surface(self.surface);
    }

    fn get_surface_color_format(adapter: &Adapter<B>, surface: &B::Surface) -> Format {
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
    */
}

/*
impl<B: Backend> Drop for RenderPresentationSurface<B> {
    fn drop(&mut self) {
        let device = self.device.upgrade().unwrap();

        unsafe {
            self.surface.unconfigure_swapchain(&device);
        }
    }
}
*/
