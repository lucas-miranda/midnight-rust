pub mod backend;
pub mod graphics;
pub mod shaders;

mod graphic_adapter;
pub use graphic_adapter::GraphicAdapter;

mod graphic_adapter_init_error;
pub use graphic_adapter_init_error::GraphicAdapterInitError;

type ActiveApi = wgpu_hal::api::Vulkan;
