use std::collections::{HashMap, HashSet};

use crate::rendering::shaders::{
    ShaderInfo,
    ShaderId,
};

#[derive(Default)]
pub struct ShaderResources {
    identifiers: HashMap<ShaderId, ShaderResource>,
    registry: HashMap<String, ShaderResource>,
}

impl ShaderResources {
    pub(crate) fn insert(&mut self, id: ShaderId) {
        self.identifiers.insert(id, ShaderResource { id });
    }

    pub fn register<S: ShaderInfo + 'static>(&mut self, shader: S, name: String) {
        self.registry.insert(name, ShaderResource { id: shader.id() });
    }

    pub fn get_default(&self) -> &ShaderResource {
        self.identifiers.get(&ShaderId::default()).unwrap()
    }
}

pub struct ShaderResource {
    id: ShaderId,
}

impl ShaderInfo for ShaderResource {
    fn id(&self) -> ShaderId {
        self.id
    }
}
