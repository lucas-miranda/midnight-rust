mod kind;
pub use kind::ShaderStageKind;

use crate::rendering::shaders::ShaderRawData;

pub struct ShaderStageData {
    data: ShaderRawData,
}

impl ShaderStageData {
    pub(super) fn new(data: ShaderRawData) -> Self {
        Self {
            data,
        }
    }

    pub fn data(&self) -> &ShaderRawData {
        &self.data
    }
}
