use crate::{ecs::{
    component::{
        self,
        //BaseQuery,
        ComponentStrongAnyRef,
    },
    system::System,
    FrameState,
}, base::ApplicationState};
use crate::input;

#[derive(Default)]
pub struct UpdateSystem<'a> {
    phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a> UpdateSystem<'a> {
    fn component_filter(_component: &ComponentStrongAnyRef) -> bool {
        //component.borrow().as_updatable().is_some()
        true
    }
}

impl<'a> System for UpdateSystem<'a> {
    type Query<'q> = component::FnQuery;

    fn setup(&mut self) {
    }

    fn input<'q>(&mut self, _query: Self::Query<'q>, _state: &mut ApplicationState) {
    }

    fn run<'q>(&mut self, _query: Self::Query<'q>, _state: &mut FrameState) {
        //println!("[UpdateSystem] {} captured components", query.count());

        //for component_ref in query.iter_components() {
            //let mut component = component_ref.borrow_mut();

            //let updateStep: Option<&dyn UpdateStep> = component.as_any().downcast_ref();



            //component.run();
        //}
    }

    fn create_query<'q>(&self) -> Self::Query<'q> {
        Self::Query::new(Self::component_filter)
    }
}
