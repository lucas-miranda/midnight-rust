use std::rc::Weak;

use wgpu::SurfaceError;

use crate::math::Vector2;

pub struct RenderPresentationSurface {
    device: Weak<wgpu::Device>,
    adapter: wgpu::Adapter,
    surface: wgpu::Surface,
    surface_format: wgpu::TextureFormat,
    need_reconfigure_swapchain: bool,
    requested_swapchain_size: Option<(u32, u32)>,
    surface_extent: Vector2<u32>, // TODO change to Size<T>
}

impl RenderPresentationSurface {
    pub(super) fn new(
        device: Weak<wgpu::Device>,
        adapter: wgpu::Adapter,
        surface: wgpu::Surface,
        surface_format: wgpu::TextureFormat,
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

    pub(in crate::rendering) fn acquire_surface(
        &mut self
    ) -> Result<(wgpu::SurfaceTexture, wgpu::TextureView), SurfaceError> {
        // reconfigure if needed
        self.reconfigure_swapchain(false);

        let surface_texture = self.surface
            .get_current_texture()?;

        let surface_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        Ok((surface_texture, surface_view))
    }

    pub(super) fn surface(&self) -> &wgpu::Surface {
        &self.surface
    }

    pub(super) fn surface_format(&self) -> &wgpu::TextureFormat {
        &self.surface_format
    }

    pub(super) fn capabilities(&self) -> wgpu::SurfaceCapabilities {
        self.surface.get_capabilities(&self.adapter)
    }

    pub(super) fn reconfigure_swapchain(&mut self, force: bool) {
        if !self.need_reconfigure_swapchain && !force {
            return;
        }

        match self.requested_swapchain_size.take() {
            Some(ref size) => self.surface_extent = size.into(),
            None => (),
        }

        let device = self.device.upgrade().unwrap();
        let surface_caps = self.capabilities();

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_caps.formats[0],
            width: self.surface_extent.x,
            height: self.surface_extent.y,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        self.surface.configure(&device, &surface_config);

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
}
