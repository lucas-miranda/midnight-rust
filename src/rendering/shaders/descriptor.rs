use std::collections::HashMap;

use super::{
    builder::ShaderProcessor,
    ShaderStage,
    ShaderStageKind,
    ShaderFormat,
};

// TODO
// - Add support to multiple entry points per stage descriptor
//      Using something as: ShaderStageKind.Vertex | ShaderStageKind.Fragment

/// Holds every shader entry point by it's stage kind.
#[derive(Default)]
pub struct ShaderDescriptor<'a> {
    stages: HashMap<ShaderStageKind, ShaderStageDescriptor<'a>>
}

impl<'a> ShaderDescriptor<'a> {
    /// Registers a new stage entry point, using an individual file.
    pub fn with_stage(
        mut self,
        stage: ShaderStageKind,
        format: ShaderFormat,
        src: &'a str,
    ) -> Self {
        self.stages.insert(
            stage,
            ShaderStageDescriptor {
                format,
                src,
            },
        );

        self
    }

    /// Returns a registered stage descriptor, if exists.
    pub fn get_stage(&self, stage: &ShaderStageKind) -> Option<&ShaderStageDescriptor> {
        self.stages.get(stage)
    }

    /// Asks provided [`ShaderProcessor`] to process a stage and returns it's result.
    pub(super) fn process_stage(
        &self,
        stage: &ShaderStageKind,
        processor: &ShaderProcessor
    ) -> Option<ShaderStage> {
        match self.get_stage(stage) {
            Some(d) => Some(processor.process(stage, d)),
            None => None,
        }
    }
}

/// Describes a shader stage, by holding it's format and source.
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
