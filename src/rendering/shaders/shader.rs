use std::hash::{
    Hash,
    Hasher,
};

use super::ShaderStage;

pub type ShaderId = u32;

pub struct Shader {
    id: ShaderId,
    vertex: ShaderStage,
    fragment: ShaderStage,
}

impl Shader {
    pub(super) fn new(id: ShaderId, vertex: ShaderStage, fragment: ShaderStage) -> Self {
        Self {
            id,
            vertex,
            fragment,
        }
    }

    pub fn id(&self) -> ShaderId {
        self.id
    }

    pub fn vertex(&self) -> &ShaderStage {
        &self.vertex
    }

    pub fn fragment(&self) -> &ShaderStage {
        &self.fragment
    }
}

impl Hash for Shader {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
