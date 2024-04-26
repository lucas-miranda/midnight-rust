use num_traits::Num;

use crate::{
    base::ApplicationState,
    components::{GraphicDisplayer, transform::Transform},
    ecs::entity::Entities,
    math::{Vector2, Size2},
    rendering::{Vertex2D, VertexPosition, VertexColor, graphics::Graphic, Vertex2DTextureColor, Color},
};

use super::UIComponent;

pub struct Frame {
    pos: Vector2<f32>,
    size: Size2<f32>,
    children: Vec<Box<dyn UIComponent>>,
    graphic: Option<Box<dyn Graphic<Vertex2D>>>,
    color: Color<f32>,
}

impl Frame {
    pub fn new(size: Size2<f32>) -> Self {
        Self {
            pos: Default::default(),
            size,
            children: Default::default(),
            graphic: None,
            color: Color::<f32>::WHITE,
        }
    }

    pub fn pos(&self) -> Vector2<f32> {
        self.pos
    }

    pub fn add<U: 'static + UIComponent>(&mut self, ui_component: U) -> &mut Self {
        self.children.push(Box::new(ui_component));

        self
    }

    pub fn at_pos(mut self, pos: Vector2<f32>) -> Self {
        self.pos = pos;

        self
    }

    pub fn with_graphic<G: 'static + Graphic<Vertex2D>>(mut self, g: G) -> Self {
        self.graphic = Some(Box::new(g));

        self
    }

    pub fn with_color<C: Into<Color<f32>>>(mut self, color: C) -> Self {
        self.color = color.into();

        self
    }

    /*
    pub fn make_entity(
        self,
        entities: &mut Entities,
        app_state: &ApplicationState,
    ) {
        let mut frame = entities.create();

        if let Some(transform_strong_ref) = frame.components().get::<Transform>() {
            let mut transform = transform_strong_ref.borrow_mut();
            transform.local_position = self.pos;
        }

        frame.register_component({
            //let mut graphic_displayer = GraphicDisplayer::<Vertex2D>::default();
            let frame_graphic = UIFrameGraphic::new(self.size);

            for child in self.children {
                //child.make_entity()
            }

            let graphic_displayer = GraphicDisplayer::new(frame_graphic);


            // TODO  write a Graphic which translates UI tree to vertices, textures and shaders
            //       create a Graphic for Frame

            graphic_displayer
        });

        frame.build()
    }
    */
}

impl UIComponent for Frame {
    fn push_vertices(&self, vertices: &mut Vec<Vertex2DTextureColor>) {
        vertices.append(&mut vec!(
            Vertex2DTextureColor::from_position(self.pos + Vector2::zero()).with_color(self.color),
            Vertex2DTextureColor::from_position(self.pos + Vector2::new(self.size.width, 0.0)).with_color(self.color),
            Vertex2DTextureColor::from_position(self.pos + Vector2::new(0.0, self.size.height)).with_color(self.color),

            Vertex2DTextureColor::from_position(self.pos + Vector2::new(0.0, self.size.height)).with_color(self.color),
            Vertex2DTextureColor::from_position(self.pos + Vector2::new(self.size.width, 0.0)).with_color(self.color),
            Vertex2DTextureColor::from_position(self.pos + Vector2::new(self.size.width, self.size.height)).with_color(self.color),
        ));

        for child in &self.children {
            child.push_vertices(vertices);
        }
    }
}


