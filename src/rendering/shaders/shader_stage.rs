#[non_exhaustive]
pub enum ShaderData {
    SpirV(Vec<u32>),
    #[cfg(feature = "shader-naga")]
    Naga(gfx_hal::device::NagaShader),
}

pub struct ShaderStage {
    data: ShaderData,
}

impl ShaderStage {
    pub(super) fn new(data: ShaderData) -> Self {
        Self {
            data,
        }
    }

    pub fn data(&self) -> &ShaderData {
        &self.data
    }
}
