use serde::{Serialize, Deserialize};

use crate::ecs::component_lib::{Health, Speed, PathingRadius, AutoAttackCooldown, AttackDamage, MissleSpeed};

#[derive(Debug, Serialize, Deserialize)]
pub struct AABB3DInfo {
  pub height: f32,
  pub radius: f32
}

#[derive(Serialize, Deserialize)]
//used as an intermediary struct for loading in champ data
pub struct Champion{
  //basic info
  pub health:Health,
  
  //movement and collision info
  pub speed: Speed,
  pub selection_radius: AABB3DInfo,
  pub pathing_radius: PathingRadius,

  //combat info
  pub auto_attack_missle_speed: MissleSpeed,
  pub auto_attack_cooldown: AutoAttackCooldown,
  pub attack_damage: AttackDamage
}