use std::collections::{HashMap, HashSet};

use crate::rendering::shaders::{
    ShaderInfo,
    Shader,
};

#[derive(Default)]
pub struct ShaderResources {
    identifiers: HashMap<Shader, ShaderResource>,
    registry: HashMap<String, ShaderResource>,
}

impl ShaderResources {
    pub(crate) fn insert(&mut self, shader: Shader) {
        self.identifiers.insert(shader, ShaderResource { shader });
    }

    pub fn register<S: ShaderInfo + 'static>(&mut self, shader: S, name: String) {
        self.registry.insert(name, ShaderResource { shader: shader.identifier() });
    }

    /*
    pub fn get_default(&self) -> &ShaderResource {
        self.identifiers.get(&ShaderId::default()).unwrap()
    }
    */
}

pub struct ShaderResource {
    shader: Shader,
}

/*
impl ShaderInfo for ShaderResource {
    fn id(&self) -> ShaderId {
        self.id
    }
}
*/
