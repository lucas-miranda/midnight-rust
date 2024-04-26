mod frame;

use std::{rc::{Rc, Weak}, cell::RefCell};

pub use frame::*;

mod ui_graphic;
pub use ui_graphic::*;

use crate::{
    base::ApplicationState,
    components::GraphicDisplayer,
    ecs::{
        component::{self, ComponentAttribute, Component, Components, ComponentRef, QueryEntry, BaseQuery, ComponentStrongRef},
        system::System,
        FrameState,
    },
    rendering::Vertex2DTextureColor,
};

pub trait UIComponent {
    fn push_vertices(&self, vertices: &mut Vec<Vertex2DTextureColor>);
}

//

// it should be splitted into two main components:
// 1. A component which will take care of logics
//    and will update the graphics one
// 2. A graphics component which will forward the graphics data
//    to the renderer
pub struct UI {
    children: Vec<Box<dyn UIComponent>>,
    displayer: Option<ComponentRef<GraphicDisplayer<Vertex2DTextureColor>>>,
}

impl UI {
    pub fn new() -> Self {
        Self {
            children: Default::default(),
            displayer: None,
        }
    }

    pub fn displayer(&self) -> Option<ComponentStrongRef<GraphicDisplayer<Vertex2DTextureColor>>> {
        self.displayer
            .as_ref()
            .map(|c| c.retrieve().ok())?
    }

    pub fn mut_displayer(&mut self) -> Option<ComponentStrongRef<GraphicDisplayer<Vertex2DTextureColor>>> {
        self.displayer
            .as_mut()
            .map(|c| c.retrieve().ok())?
    }

    pub fn add<U: 'static + UIComponent>(&mut self, ui_component: U) -> &mut Self {
        self.children.push(Box::new(ui_component));

        self
    }
}

impl Component for UI {
    fn attributes(&self) -> ComponentAttribute {
        ComponentAttribute::None
    }

    fn registered(&mut self, components: &mut Components) {
        let graphic = UIGraphic::new();

        let displayer = GraphicDisplayer::new(graphic);
        components.register(displayer);

        self.displayer = components.get_kind();
    }

    fn unregistered(&mut self) {
    }
}

//

#[derive(Default)]
pub struct UISystem {
}

impl System for UISystem {
    type Query<'q> = component::Query<'q, UI>;

    fn setup(&mut self) {
    }

    fn input<'q>(&mut self, _query: Self::Query<'q>, _state: &mut ApplicationState) {
    }

    fn run<'q>(&mut self, query: Self::Query<'q>, _state: &mut FrameState) {
        for QueryEntry { component: ui, .. } in query.iter_components() {
            if let Some(displayer_ref) = ui.displayer() {
                let mut displayer = displayer_ref.borrow_mut();

                if let Some(ui_graphic) = displayer.mut_retrieve_graphic::<UIGraphic>() {
                    // TODO  extract vertices from ui
                    let mut vertices = Vec::new();

                    println!("Pushing vertices from {} children", ui.children.len());
                    for child in &ui.children {
                        child.push_vertices(&mut vertices)
                    }

                    // TODO  push vertices into ui graphic
                    println!("Registering {} vertices", vertices.len());
                    ui_graphic.vertices.clear();
                    ui_graphic.vertices.append(&mut vertices);
                }
            }
        }
    }

    fn create_query<'q>(&self) -> Self::Query<'q> {
        Self::Query::default()
    }
}
