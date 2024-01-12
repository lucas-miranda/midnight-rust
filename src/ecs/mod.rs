pub mod component;

mod domain;
pub use domain::Domain;

pub mod entity;
pub mod system;

use crate::{time::DeltaTime, base::ApplicationState};

pub struct FrameState<'a> {
    pub delta: DeltaTime,
    pub app: &'a mut ApplicationState,
}
