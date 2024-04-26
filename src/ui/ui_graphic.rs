use std::marker::PhantomData;
use std::any::Any;

use image::Frame;
use wgpu::{PrimitiveState, PrimitiveTopology, PolygonMode, FrontFace};

use crate::{
    math::{Vector2, Size2},
    rendering::{
        graphics::{Graphic, GraphicDrawError},
        DrawConfig,
        RenderState,
        Vertex2D,
        VertexPosition, ShaderConfig, Vertex2DTextureColor, VertexColor, Color,
    },
};

/*
pub trait UIGraphic {
}
*/

pub struct UIGraphic {
    pub vertices: Vec<Vertex2DTextureColor>,
}

impl UIGraphic {
    pub fn new() -> Self {
        Self {
            vertices: Default::default(),
        }
    }
}

impl Graphic<Vertex2DTextureColor> for UIGraphic where {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn draw<'d>(
        &'d self,
        state: &'d mut dyn RenderState<Vertex2DTextureColor>,
        draw_config: DrawConfig<Vertex2DTextureColor>,
    ) -> Result<(), GraphicDrawError> {
        state.extend(
            self.vertices.iter(),
            None,
            draw_config,
        ).map_err(GraphicDrawError::from)
    }
}


/*
pub struct UIFrameGraphic {
    pub size: Size2<f32>,
    border_primitive_state: PrimitiveState,
    children: Vec<Box<dyn UIGraphic>>,
}

impl UIFrameGraphic {
    pub fn new(size: Size2<f32>) -> Self {
        Self {
            size,
            border_primitive_state: PrimitiveState {
                topology: PrimitiveTopology::LineStrip,
                polygon_mode: PolygonMode::Fill,
                //topology: PrimitiveTopology::TriangleList,
                //polygon_mode: PolygonMode::Fill,
                front_face: FrontFace::Cw,
                ..Default::default()
            },
            children: Vec::default(),
        }
    }

    pub fn register<G: 'static + UIGraphic>(&mut self, graphic: G) {
        self.children.push(Box::new(graphic));
    }

    fn apply_border_shader_changes(&self, mut config: ShaderConfig) -> ShaderConfig {
        let primitive = config.mut_primitive_state();
        *primitive = self.border_primitive_state;
        config
    }
}

impl Graphic<Vertex2DTextureColor> for UIFrameGraphic where {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn draw<'d>(
        &'d self,
        state: &'d mut dyn RenderState<Vertex2DTextureColor>,
        draw_config: DrawConfig<Vertex2DTextureColor>,
    ) -> Result<(), GraphicDrawError> {
        // push background vertices
        state.extend(
            vec!(
                Vertex2DTextureColor::from_position(Vector2::zero()).with_color(Color::<f32>::WHITE),
                Vertex2DTextureColor::from_position(Vector2::new(self.size.width, 0.0)).with_color(Color::<f32>::WHITE),
                Vertex2DTextureColor::from_position(Vector2::new(0.0, self.size.height)).with_color(Color::<f32>::WHITE),

                Vertex2DTextureColor::from_position(Vector2::new(0.0, self.size.height)).with_color(Color::<f32>::WHITE),
                Vertex2DTextureColor::from_position(Vector2::new(self.size.width, 0.0)).with_color(Color::<f32>::WHITE),
                Vertex2DTextureColor::from_position(Vector2::new(self.size.width, self.size.height)).with_color(Color::<f32>::WHITE),
            ).iter(),
            None,
            draw_config,
        ).map_err(GraphicDrawError::from)?;

        //println!("bg draw_config: {:?}", draw_config);

        let b = draw_config.clone()
                           .apply_shader_changes(|s| self.apply_border_shader_changes(s));

        //println!("border draw_config: {:?}", b);

        // push border vertices with modified shader params
        state.extend(
            vec!(
                Vertex2DTextureColor::from_position(Vector2::zero()).with_color(Color::<f32>::RED),
                Vertex2DTextureColor::from_position(Vector2::new(self.size.width, 0.0)).with_color(Color::<f32>::RED),
                Vertex2DTextureColor::from_position(Vector2::new(self.size.width, self.size.height)).with_color(Color::<f32>::RED),
                Vertex2DTextureColor::from_position(Vector2::new(0.0, self.size.height)).with_color(Color::<f32>::RED),
                Vertex2DTextureColor::from_position(Vector2::zero()).with_color(Color::<f32>::RED),
            ).iter(),
            None,
            b,
        ).map_err(GraphicDrawError::from)?;

        // children
        for child in &self.children {
            //child.draw(state, draw_config)?;
        }

        Ok(())
    }
}

impl UIGraphic for UIFrameGraphic {
}
*/
