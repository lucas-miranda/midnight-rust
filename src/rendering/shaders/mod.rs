pub mod builder;

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

pub struct Shader {
    vertex: Option<ShaderStage>,
    fragment: Option<ShaderStage>,
}

impl Shader {
    pub(super) fn new(vertex: Option<ShaderStage>, fragment: Option<ShaderStage>) -> Self {
        Self {
            vertex,
            fragment,
        }
    }

    pub fn vertex(&self) -> &Option<ShaderStage> {
        &self.vertex
    }

    pub fn fragment(&self) -> &Option<ShaderStage> {
        &self.fragment
    }
}
