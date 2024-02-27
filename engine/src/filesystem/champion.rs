use serde::{Serialize, Deserialize};
use crate::component_lib::{AttackDamage, Health, MissleSpeed, PathingRadius, UnitSpeed};

#[derive(Debug, Serialize, Deserialize)]
pub struct AABB3DInfo {
  pub height: f32,
  pub radius: f32
}

#[derive(Serialize, Deserialize)]
///An intermediary struct for loading in champ data from a JSON.
pub struct Champion{
  //Basic info
  pub health:Health,
  
  //Movement and collision info
  pub speed: UnitSpeed,
  pub selection_radius: AABB3DInfo,
  pub pathing_radius: PathingRadius,

  //Combat info
  pub auto_attack_missle_speed: MissleSpeed,
  pub auto_attack_cooldown:f64,
  pub attack_damage: AttackDamage
}