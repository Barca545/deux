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
pub struct AttackDamage(pub u32);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Armor(pub u32);

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct SpellResource(pub u32);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Health {
  pub max: u32,
  pub remaining: u32,
}
impl Health {
  pub fn new(max: u32) -> Self {
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
