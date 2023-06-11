use std::collections::HashMap;

use super::{
    builder::ShaderProcessor,
    ShaderStage,
    ShaderStageKind,
    ShaderFormat,
};

#[derive(Default)]
pub struct ShaderDescriptor<'a> {
    stages: HashMap<ShaderStageKind, ShaderStageDescriptor<'a>>
}

impl<'a> ShaderDescriptor<'a> {
    pub fn with_stage(mut self, stage: ShaderStageKind, format: ShaderFormat, src: &'a str) -> Self {
        self.stages.insert(
            stage,
            ShaderStageDescriptor {
                format,
                src,
            },
        );

        self
    }

    pub fn get_stage(&self, stage: &ShaderStageKind) -> Option<&ShaderStageDescriptor> {
        self.stages.get(stage)
    }

    pub(super) fn process_stage(&self, stage: &ShaderStageKind, processor: &ShaderProcessor) -> Option<ShaderStage> {
        match self.get_stage(stage) {
            Some(d) => Some(processor.process(stage, d)),
            None => None,
        }
    }
}

pub struct ShaderStageDescriptor<'a> {
    format: ShaderFormat,
    src: &'a str,
}

impl<'a> ShaderStageDescriptor<'a> {
    pub fn format(&self) -> &ShaderFormat {
        &self.format
    }

    pub(super) fn src(&self) -> &str {
        &self.src
    }
}
