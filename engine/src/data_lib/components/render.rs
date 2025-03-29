use crate::renderer::ModelId;

#[derive(Debug, Clone, Copy)]
pub struct SkinnedRenderable(pub ModelId);

#[derive(Debug, Clone, Copy)]
pub struct StaticRenderable(pub ModelId);

#[derive(Debug, Clone, Copy)]
pub struct DebugModel(pub ModelId);
