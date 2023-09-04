use std::borrow::Cow;

/// Ready to execute shader file contents.
#[non_exhaustive]
pub enum ShaderRawData {
    Wgsl(String),

    #[cfg(feature = "shader-shaderc")]
    SpirV(Vec<u32>),

    #[cfg(feature = "shader-naga")]
    Naga(gfx_hal::device::NagaShader),
}

impl<'a> From<&'a ShaderRawData> for wgpu::ShaderSource<'a> {
    fn from(shader_raw_data: &'a ShaderRawData) -> Self {
        match shader_raw_data {
            ShaderRawData::Wgsl(wgsl) => wgpu::ShaderSource::Wgsl(Cow::Borrowed(wgsl)),

            #[cfg(feature = "shader-shaderc")]
            ShaderRawData::SpirV(spirv) => wgpu::ShaderSource::SpirV(Cow::Borrowed(spirv)),

            #[cfg(feature = "shader-naga")]
            ShaderRawData::Naga(naga_shader) => {
                unimplemented!();
            },
        }
    }
}
