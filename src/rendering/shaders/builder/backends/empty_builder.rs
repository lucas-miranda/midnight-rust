use crate::rendering::shaders::{
    ShaderStageKind,
    ShaderStageData,
};

use super::{
    GLSLShaderProcessor,
    ShaderBackendError,
};

pub struct EmptyBuilderBackend {
}

impl GLSLShaderProcessor for EmptyBuilderBackend {
    fn build(&self, _stage: ShaderStageKind, _src: &str) -> Result<ShaderStageData, ShaderBackendError> {
        Err(ShaderBackendError::NullResultObject("Empty builder.".to_owned()))
    }
}
