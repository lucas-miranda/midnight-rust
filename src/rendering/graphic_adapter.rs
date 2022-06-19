use crate::window::Window;
use super::backend::GraphicBackend;

pub type Result<T> = std::result::Result<T, super::GraphicAdapterInitError>;

pub struct GraphicAdapter {
    backend: GraphicBackend,
}

impl GraphicAdapter {
    pub fn new(window: &Window) -> Result<Self> {
        Self::with_surface_size(window, (320, 180))
    }

    pub fn with_surface_size(window: &Window, size: (u32, u32)) -> Result<Self> {
        let backend = GraphicBackend::new(window, size)
            .unwrap();
            //.map_err(super::GraphicAdapterInitError::BackendFailed)?;

         Ok(Self {
             backend,
         })
    }

    pub fn request_resize_surface(&mut self, width: u32, height: u32) {
        self.backend
            .mut_render_backend()
            .mut_presentation_surface()
            .request_reconfigure_swapchain_with(width, height);
    }

    pub fn render(&mut self) {
        self.backend
            .mut_render_backend()
            .render()
            .unwrap()
    }
}
