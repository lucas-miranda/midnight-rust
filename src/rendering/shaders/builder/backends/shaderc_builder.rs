use crate::rendering::shaders::{
    builder::backends::{
        GLSLShaderProcessor,
        ShaderBackendError,
        ShaderStageKind,
    },
    ShaderRawData,
    ShaderStageData,
};

pub struct ShadercBuilderBackend {
    compiler: shaderc::Compiler,
}

impl Default for ShadercBuilderBackend {
    fn default() -> Self {
        Self {
            compiler: shaderc::Compiler::new().unwrap(),
        }
    }
}

impl GLSLShaderProcessor for ShadercBuilderBackend {
    fn build(&self, stage: ShaderStageKind, src: &str) -> Result<ShaderStageData, ShaderBackendError> {
        let options = shaderc::CompileOptions::new().unwrap();

        let compiled = self.compiler
            .compile_into_spirv(
                src,
                stage.into(),
                //shaderc::ShaderKind::Vertex,
                "unnamed",
                "main",
                Some(&options),
            )
            .map_err(From<shaderc::Error>::from)?;

        ShaderStageData::new(ShaderRawData::SpirV(compiled.as_binary().to_vec()))
    }
}

impl From<shaderc::Error> for ShaderBackendError {
    fn from(err: shaderc::Error) -> Self {
        match err {
            shaderc::Error::CompilationError(err_count, detailed) => Self::CompilationError { err_count, detailed },
            shaderc::Error::InternalError(msg) => Self::InternalError(msg),
            shaderc::Error::InvalidStage(msg) => Self::InvalidStage(msg),
            shaderc::Error::InvalidAssembly(msg) => Self::InvalidAssembly(msg),
            shaderc::Error::NullResultObject(msg) => Self::NullResultObject(msg),
        }
    }
}
