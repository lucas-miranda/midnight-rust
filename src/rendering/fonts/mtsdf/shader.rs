use std::{
    rc::Rc,
    cell::RefCell
};

use bytemuck::{Pod, Zeroable};
use miette::IntoDiagnostic;

use crate::{
    math::Matrix4x4,
    rendering::{
        shaders::{
            AttributeFormat,
            Bindings,
            BindingsDescriptorEntry,
            BindingsError,
            Shader,
            ShaderDescriptor,
            ShaderFormat,
            ShaderInfo,
            ShaderInstance,
            ShaderStageKind,
            VertexAttribute,
            WorldViewProjectionUniforms,
        },
        Color,
        FrontFace,
        GraphicAdapter,
        PolygonMode,
        PrimitiveState,
        PrimitiveTopology,
        SamplerBindingType,
        ShaderConfig,
        TextureSampleType,
        TextureViewDimension,
    },
    vertex_attrs, resources::Asset,
};

#[repr(C)]
#[derive(Copy, Clone, Default, Pod, Zeroable)]
pub struct Uniforms {
    pub view: Matrix4x4<f32>,
    pub color: Color<f32>,
    pub screen_px_range: f32,
}

impl WorldViewProjectionUniforms for Uniforms {
    fn view(&self) -> &Matrix4x4<f32> {
        &self.view
    }

    fn mut_view(&mut self) -> &mut Matrix4x4<f32> {
        &mut self.view
    }
}

pub struct MTSDFShader {
    shader: Shader,
    uniforms: Vec<Uniforms>,
    default_config: ShaderConfig,
}

impl MTSDFShader {
    pub fn new(graphic_adapter: &mut GraphicAdapter) -> Rc<RefCell<Self>> {
        graphic_adapter
            .shader_builder()
            .create(
                ShaderDescriptor::default()
                    .with_stage(ShaderStageKind::Vertex,    ShaderFormat::WGSL, include_str!("res/vert.wgsl"))
                    .with_stage(ShaderStageKind::Fragment,  ShaderFormat::WGSL, include_str!("res/frag.wgsl"))
            )
            .set_vertex_attributes(vertex_attrs![
                Float32x2,
                Float32x2,
            ].into_iter())
            .bindings(vec![
                BindingsDescriptorEntry::uniform::<Uniforms>(),
                BindingsDescriptorEntry::texture(TextureSampleType::Float { filterable: true }, false, TextureViewDimension::D2),
                BindingsDescriptorEntry::sampler(SamplerBindingType::Filtering),
            ].into_iter())
            .build()
            .into_diagnostic()
            .unwrap()
    }

    pub fn default_config(&self) -> &ShaderConfig {
        &self.default_config
    }

    pub fn uniforms_mut(&mut self) -> &mut Uniforms {
        self.uniforms.get_mut(0).unwrap()
    }
}

impl ShaderInstance for MTSDFShader {
    fn new(shader: Shader) -> Self {
        let default_config = ShaderConfig::new(
            &shader,
            PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Cw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: PolygonMode::Fill,
                conservative: false,
            }
        );

        Self {
            shader,
            uniforms: vec![Uniforms::default()],
            default_config,
        }
    }

    fn world_view_projection_uniforms(&self) -> Option<&dyn WorldViewProjectionUniforms> {
        Some(&self.uniforms[0])
    }

    fn mut_world_view_projection_uniforms(&mut self) -> Option<&mut dyn WorldViewProjectionUniforms> {
        Some(&mut self.uniforms[0])
    }

    fn bindings<'b>(&'b self, mut bindings: Bindings<'b>) -> Result<Bindings<'b>, BindingsError> {
        bindings.uniforms(&self.uniforms)?;

        Ok(bindings)
    }
}

impl ShaderInfo for MTSDFShader {
    fn identifier(&self) -> Shader {
        self.shader
    }
}

impl AsRef<dyn ShaderInstance> for MTSDFShader {
    fn as_ref(&self) -> &(dyn ShaderInstance + 'static) {
        self
    }
}
