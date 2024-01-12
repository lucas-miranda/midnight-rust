use std::time::Duration;


#[derive(Default)]
pub struct Diagnostics {
    pub update_timer: Duration,
    pub render_timer: Duration,
    pub draw_calls: usize,
}
