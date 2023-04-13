use std::borrow::Cow;

#[non_exhaustive]
pub enum ShaderRawData {
    SpirV(Vec<u32>),
    //#[cfg(feature = "shader-naga")]
    //Naga(gfx_hal::device::NagaShader),
}

pub struct ShaderStage {
    data: ShaderRawData,
}

impl ShaderStage {
    pub(super) fn new(data: ShaderRawData) -> Self {
        Self {
            data,
        }
    }

    pub fn data(&self) -> &ShaderRawData {
        &self.data
    }
}

impl<'a> From<&'a ShaderRawData> for wgpu::ShaderSource<'a> {
    fn from(shader_raw_data: &'a ShaderRawData) -> Self {
        match shader_raw_data {
            ShaderRawData::SpirV(spirv) => wgpu::ShaderSource::SpirV(Cow::Borrowed(spirv)),
            #[cfg(feature = "shader-naga")]
            ShaderRawData::Naga(naga_shader) => {
                unimplemented!();
            },
        }
    }
}
