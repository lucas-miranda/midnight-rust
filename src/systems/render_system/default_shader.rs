use std::{
    rc::Rc,
    cell::RefCell
};

use bytemuck::{Pod, Zeroable};

use crate::{
    math::Matrix4x4,
    rendering::{
        shaders::{
            AttributeFormat,
            Bindings,
            BindingsDescriptorEntry,
            Shader,
            ShaderDescriptor,
            ShaderFormat,
            ShaderId,
            ShaderInfo,
            ShaderInstance,
            ShaderStageKind,
            ShaderUniformInstance,
            VertexAttribute,
        },
        GraphicAdapter,
        Color,
        ShaderConfig,
        FrontFace,
        PolygonMode,
        PrimitiveState,
        PrimitiveTopology,
    },
    vertex_attrs,
};

#[repr(C)]
#[derive(Copy, Clone, Default, Pod, Zeroable)]
pub struct DefaultUniforms {
    pub view: Matrix4x4<f32>,
    pub color: Color<f32>,
}

pub struct DefaultShader {
    shader: Shader,
    uniforms: Vec<DefaultUniforms>,
    default_config: ShaderConfig,
}

impl DefaultShader {
    pub fn new(graphic_adapter: &Rc<RefCell<GraphicAdapter>>) -> Rc<RefCell<Self>> {
        graphic_adapter
            .borrow_mut()
            .shader_builder()
            .create::<DefaultUniforms>(
                ShaderDescriptor::default()
                    .with_stage(ShaderStageKind::Vertex,    ShaderFormat::GLSL, include_str!("shaders/p1.vert"))
                    .with_stage(ShaderStageKind::Fragment,  ShaderFormat::GLSL, include_str!("shaders/p1.frag"))
            )
            .set_vertex_attributes(vertex_attrs![
                Float32x2,
            ].into_iter())
            .bindings(vec![
                BindingsDescriptorEntry::Uniform(std::marker::PhantomData),
            ].into_iter())
            .build()
    }

    pub fn default_config(&self) -> &ShaderConfig {
        &self.default_config
    }

    pub fn uniforms_mut(&mut self) -> &mut DefaultUniforms {
        self.uniforms.get_mut(0).unwrap()
    }
}

impl ShaderInstance for DefaultShader {
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
            uniforms: vec![DefaultUniforms::default()],
            default_config,
        }
    }

    fn uniforms_as_slice<'s>(&'s self) -> &'s [u8] {
        bytemuck::cast_slice(self.uniforms.as_slice())
    }

    fn bindings<'b>(&'b self, mut bindings: Bindings<'b>) -> Bindings<'b> {
        bindings.push_uniforms(&self.uniforms);

        bindings
    }
}

impl ShaderUniformInstance for DefaultShader {
    type Uniforms = DefaultUniforms;

    fn uniforms(&self) -> &Self::Uniforms {
        self.uniforms.get(0).unwrap()
    }
}

impl ShaderInfo for DefaultShader {
    fn id(&self) -> ShaderId {
        self.shader.id()
    }
}

impl AsRef<dyn ShaderInstance> for DefaultShader {
    fn as_ref(&self) -> &(dyn ShaderInstance + 'static) {
        self
    }
}
