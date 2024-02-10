use serde::{Serialize, Deserialize};
use crate::component_lib::{AttackDamage, AutoAttackCooldown, Health, MissleSpeed, PathingRadius, UnitSpeed};

#[derive(Debug, Serialize, Deserialize)]
pub struct AABB3DInfo {
  pub height: f32,
  pub radius: f32
}

#[derive(Serialize, Deserialize)]
///An intermediary struct for loading in champ data from a JSON.
pub struct Champion{
  //basic info
  pub health:Health,
  
  //movement and collision info
  pub speed: UnitSpeed,
  pub selection_radius: AABB3DInfo,
  pub pathing_radius: PathingRadius,

  //combat info
  pub auto_attack_missle_speed: MissleSpeed,
  pub auto_attack_cooldown: AutoAttackCooldown,
  pub attack_damage: AttackDamage
}