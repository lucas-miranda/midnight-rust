pub mod backend;
pub mod graphics;
pub mod shaders;

mod graphic_adapter;
pub use graphic_adapter::GraphicAdapter;

mod graphic_adapter_init_error;
pub use graphic_adapter_init_error::GraphicAdapterInitError;

mod draw_config;
pub use draw_config::DrawConfig;

mod color;
pub use color::Color;
