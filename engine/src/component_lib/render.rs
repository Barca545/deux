use crate::view::ModelId;
#[derive(Debug, Clone, Copy)]
pub struct PlayerModel(pub ModelId);

#[derive(Debug, Clone, Copy)]
pub struct AutoAttackModel(pub ModelId);

#[derive(Debug, Clone, Copy)]
pub struct DebugModel(pub ModelId);

#[derive(Debug, Clone, Copy)]
pub struct StaticModel(pub ModelId);
