use crate::rendering::shaders::{
    builder::backends::{
        ShaderBuilderBackend,
        ShaderGLSLBackendProcessor,
        ShaderStageKind,
    },
    Shader,
    ShaderId,
    ShaderRawData,
    ShaderStage,
};

pub type Backend = ShadercBuilderBackend;

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

impl ShaderBuilderBackend for ShadercBuilderBackend {
    type GLSL = Self;

    fn glsl(&self) -> &Self::GLSL {
        self
    }
}

impl ShaderGLSLBackendProcessor for ShadercBuilderBackend {
    //fn build(&self, id: ShaderId, vertex: &str, fragment: &str) -> Shader {
    fn build(&self, stage: ShaderStageKind, src: &str) -> ShaderStage {
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
            .expect(format!("Failed to compile {} shader", stage).as_str());

        /*
        let compiled_vertex = self.compiler
            .compile_into_spirv(
                vertex,
                shaderc::ShaderKind::Vertex,
                "unnamed",
                "main",
                Some(&options),
            )
            .expect("Failed to compile vertex shader");

        let compiled_fragment = self.compiler
            .compile_into_spirv(
                fragment,
                shaderc::ShaderKind::Fragment,
                "unnamed",
                "main",
                Some(&options),
            )
            .expect("Failed to compile fragment shader");

        Shader::new(
            id,
            ShaderStage::new(ShaderRawData::SpirV(compiled_vertex.as_binary().to_vec())),
            ShaderStage::new(ShaderRawData::SpirV(compiled_fragment.as_binary().to_vec())),
        )
        */

        ShaderStage::new(ShaderRawData::SpirV(compiled.as_binary().to_vec()))
    }
}
