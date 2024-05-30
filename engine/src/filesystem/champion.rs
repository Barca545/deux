use crate::data_lib::{GameplayRadius, PathingRadius};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AABB3DInfo {
  pub height:f32,
  pub radius:f32
}

#[derive(Serialize, Deserialize)]
///An intermediary struct for loading in champ data from a JSON.
pub struct Champion {
  //Basic info
  pub health:i32,
  pub resource:i32,

  //Defensive
  pub armor:i32,
  pub magic_resist:i32,

  //Offensive
  pub attack_damage:i32,
  pub magic_damage:i32,
  pub auto_attack_missle_speed:f32,
  pub auto_attack_cooldown:f64,
  // pub attack_speed: i32,

  //Movement and collision info
  pub unit_speed:f32,
  pub selection_radius:AABB3DInfo,
  pub pathing_radius:PathingRadius,
  pub gameplay_radius:GameplayRadius
}
