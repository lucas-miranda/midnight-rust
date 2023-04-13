use std::rc::Rc;

use crate::window::Window;

use super::{
    RenderBackendBuildError,
    RenderBackend,
    RenderPresentationSurface,
};

pub type Result<T> = std::result::Result<T, RenderBackendBuildError>;

pub struct RenderBackendBuilder<'a> {
    window: &'a Window,
    surface_size: (u32, u32),
}

impl<'a> RenderBackendBuilder<'a> {
    pub(crate) fn new(window: &'a Window, surface_size: (u32, u32)) -> Self {
        Self {
            window,
            surface_size,
        }
    }

    pub fn build(self) -> Result<RenderBackend> {
        let app_name = "app name";

        let instance = Self::create_instance(app_name);
        let surface = Self::create_surface(&instance, self.window)?;
        let adapter = pollster::block_on(Self::find_adapter(&instance, &surface))?;

        let surface_caps = surface.get_capabilities(&adapter);
        println!("Surface caps: {:#?}", surface_caps);

        let (device, queue) = pollster::block_on(Self::open_logical_device(&adapter))?;

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_caps.formats[0],
            width: self.surface_size.0,
            height: self.surface_size.1,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &surface_config);

        let device = Rc::new(device);
        let weak_device = Rc::downgrade(&device);

        Ok(RenderBackend::new(
            instance,
            device,
            surface_caps,
            queue,
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

    fn create_instance(_name: &str) -> wgpu::Instance {
        let instance_desc = wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            dx12_shader_compiler: wgpu::Dx12Compiler::Fxc,
        };

        wgpu::Instance::new(instance_desc)
    }

    fn create_surface(instance: &wgpu::Instance, window: &Window) -> Result<wgpu::Surface> {
        unsafe { instance.create_surface(&window) }
            .map_err(RenderBackendBuildError::SurfaceFailed)
    }

    async fn find_adapter(instance: &wgpu::Instance, surface: &wgpu::Surface) -> Result<wgpu::Adapter> {
        instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(surface),
            })
            .await
            .ok_or_else(|| RenderBackendBuildError::AdapterNotFound)
    }

    async fn open_logical_device(adapter: &wgpu::Adapter) -> Result<(wgpu::Device, wgpu::Queue)> {
        adapter.request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::downlevel_webgl2_defaults()
                        .using_resolution(adapter.limits()),

                },
                None
            )
            .await
            .map_err(RenderBackendBuildError::LogicalDeviceOpenFailed)
    }
}
