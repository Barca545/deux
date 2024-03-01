use crate::event::{AbilityFour, AbilityOne, AbilityThree, AbilityTwo, AutoAttack as AutoAttackId};
use serde::{Deserialize, Serialize};
use std::{
  any::{Any, TypeId},
  collections::HashMap,
};

// Refactor:
// -Ability map should probably be stored on the entity not as a resource
// -Ability map needs the ability to add/update scripts
// -Ability map scripts probably will need to hold multiple data, they will have spawn logic, some will have on hit logic, and some will have on tick logic

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
///Component containing the speed of a missle entity.
pub struct MissleSpeed(pub f32);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
///Component containing the attack damage of an entity.
pub struct AttackDamage(pub i32);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Armor(pub i32);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Health {
  pub max: i32,
  pub remaining: i32,
}
impl Health {
  pub fn new(max: i32) -> Self {
    Health { max, remaining: max }
  }
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
///Component which tracks the kills, deaths, and assits of an entity.
pub struct KDA {
  kills: u32,
  deaths: u32,
  assists: u32,
}
impl KDA {
  ///Increments the tracked kills by 1.
  pub fn kill(&mut self, number: u32) {
    self.kills += number;
  }

  ///Increments the tracked deaths by 1.
  pub fn death(&mut self, number: u32) {
    self.deaths += number;
  }

  ///Increments the tracked assists by 1.
  pub fn assist(&mut self, number: u32) {
    self.assists += number;
  }
}

pub struct AbilityMap {
  map: HashMap<TypeId, String>,
}

impl AbilityMap {
  pub fn new() -> Self {
    let mut map = HashMap::new();
    map.insert(AbilityOne.type_id(), String::from("Ability One"));
    map.insert(AbilityTwo.type_id(), String::from("Ability Two"));
    map.insert(AbilityThree.type_id(), String::from("Ability Three"));
    map.insert(AbilityFour.type_id(), String::from("Ability Four"));
    map.insert(AutoAttackId.type_id(), String::from("Auto Attack"));
    AbilityMap { map }
  }

  pub fn get(&self, id: TypeId) -> &str {
    self.map.get(&id).unwrap()
  }
}
