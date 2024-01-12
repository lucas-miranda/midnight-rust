use std::time::Duration;

use crate::{
    base::ApplicationState,
    components::{
        transform::Transform,
        DiagComponent,
        GraphicDisplayer,
    },
    ecs::{
        component::{
            self,
            QueryEntry,
            BaseQuery,
        },
        system::System,
        FrameState,
    },
    math::Vector2,
    rendering::{
        fonts::MTSDFFontRendering,
        graphics::Text,
        Vertex2DTexture,
    },
};

pub struct DiagSystem {
    fps: u32,
    frame_count: u32,
    remaining_duration: Duration,
}

impl System for DiagSystem {
    type Query<'q> = (
        component::MutQuery<'q, DiagComponent>,
        component::MutQuery<'q, GraphicDisplayer<Vertex2DTexture>>,
        component::MutQuery<'q, Transform>,
    );

    fn setup(&mut self) {
    }

    fn input<'q>(&mut self, _query: Self::Query<'q>, _state: &mut ApplicationState) {
    }

    fn run<'q>(&mut self, query: Self::Query<'q>, state: &mut FrameState) {
        self.frame_count += 1;
        self.remaining_duration = self.remaining_duration.saturating_sub(state.delta.duration);

        if self.remaining_duration.is_zero() {
            // frame count completed
            self.fps = self.frame_count;
            self.frame_count = 0;
            self.remaining_duration = Duration::from_secs(1);
        }

        for QueryEntry { component: (a, b, c), .. } in query.iter_components() {
            if let Some(mut graphic_displayer) = b {
                if let Some(mut diag) = a {
                    diag.fps = self.fps;

                    // update graphic, if there is any
                    if let Some(ref mut g) = graphic_displayer.graphic {
                        let text: &mut Text<MTSDFFontRendering, Vertex2DTexture> = g.as_any_mut().downcast_mut().unwrap();

                        text.change_value(format!(
                            "{}\nu: {:.4}\nr: {:.4}\ndc: {}",
                            self.fps.to_string(),
                            state.app.diagnostics.update_timer.as_secs_f32(),
                            state.app.diagnostics.render_timer.as_secs_f32(),
                            state.app.diagnostics.draw_calls,
                        ));

                        // reposition
                        if let Some(mut transform) = c {
                            let window_size = state.app.main_window.inner_size();

                            transform.local_position
                                = Vector2::new(
                                    window_size.width as f32 - text.px_size().width,
                                    8 as f32
                                ).convert()
                        }
                    }
                }
            }
        }
    }

    fn create_query<'q>(&self) -> Self::Query<'q> {
        Self::Query::default()
    }
}

impl Default for DiagSystem {
    fn default() -> Self {
        Self {
            fps: 0,
            frame_count: 0,
            remaining_duration: Duration::from_secs(1),
        }
    }
}

