use std::fmt::Display;

#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum ShaderStageKind {
    Vertex,
    Fragment,
}

impl Display for ShaderStageKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Vertex    => write!(f, "Vertex"),
            Self::Fragment  => write!(f, "Fragment"),
        }
    }
}

#[cfg(feature = "shader-shaderc")]
impl From<ShaderStageKind> for shaderc::ShaderKind {
    fn from(value: ShaderStageKind) -> Self {
        match value {
            ShaderStageKind::Vertex => shaderc::ShaderKind::Vertex,
            ShaderStageKind::Fragment => shaderc::ShaderKind::Fragment,
        }
    }
}
