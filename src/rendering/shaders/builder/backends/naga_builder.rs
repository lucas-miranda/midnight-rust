pub type Backend = NagaBuilderBackend;

pub struct NagaBuilderBackend {
}

impl NagaBuilderBackend {
}

impl BuilderBackend for NagaBuilderBackend {
}


pub struct NagaBuilderBackend {
}

impl NagaBuilderBackend {
}

impl BuilderBackend for NagaBuilderBackend {
}


#[derive(Default)]
pub struct NagaGLSLBuilder {
    parser: naga::front::glsl::Parser,
}

impl NagaGLSLBuilder {
    pub fn build(&self, source: &str) -> Shader {
        let module = self.parser
            .parse(
                &naga::front::glsl::Options::from(naga::ShaderStage::Vertex),
                source
            )
            .unwrap();

        let capabilities = naga::valid::Capabilities::all();
        let info = naga::valid::Validator::new(
            naga::valid::ValidationFlags::all(),
            capabilities
        )
            .validate(&module)
            .expect("Naga module validation failed");

        Shader::new(ShaderData::Naga(NagaShader {
            module,
            info,
        }))
    }


    /*
    fn compile_shader(source: &str, stage: naga::ShaderStage) -> Vec<u32> {
        use naga::back::spv;

        let mut parser = naga::front::glsl::Parser::default();
        println!("Compiling shader {:?}...", stage);

        let module = parser
            .parse(
                &naga::front::glsl::Options {
                    stage,
                    defines: Default::default(),
                },
                source
            )
            .unwrap();

        //

        let capabilities = naga::valid::Capabilities::all();
        let info = naga::valid::Validator::new(
            naga::valid::ValidationFlags::all(),
            capabilities
        )
            .validate(&module)
            .expect("Naga module validation failed");

        let mut flags = spv::WriterFlags::LABEL_VARYINGS;
        flags.set(spv::WriterFlags::DEBUG, false);
        //flags.set(spv::WriterFlags::FORCE_POINT_SIZE, true);
        //flags.set(spv::WriterFlags::CLAMP_FRAG_DEPTH, true);

        let options = spv::Options {
            //lang_version: (1, 1),
            flags,
            //capabilities: None,
            //bounds_check_policies: naga::proc::BoundsCheckPolicies::default(),
            ..spv::Options::default()
        };

        let ep = &module.entry_points[0];
        let pipeline_options = spv::PipelineOptions {
            entry_point: ep.name.clone(),
            shader_stage: ep.stage,
        };

        spv::write_vec(&module, &info, &options, Some(&pipeline_options))
            .unwrap()
    }
    */
}
