use serde::{Deserialize, Serialize};
use std::vec::Drain;

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

#[derive(Debug, Clone)]
pub enum DamageType {
  True { owner: usize, damage: i32 },
  Magic { owner: usize, damage: i32 },
  Physical { owner: usize, damage: i32 },
}

///Component holding all the [`Damage`] its owner recieved during a tick.
#[derive(Debug, Clone)]
pub struct IncomingDamage(pub Vec<DamageType>);
impl IncomingDamage {
  pub fn new() -> Self {
    IncomingDamage(Vec::default())
  }

  pub fn push(&mut self, damage: DamageType) {
    self.0.push(damage);
  }

  pub fn clear(&mut self) {
    self.0.clear()
  }

  pub fn drain(&mut self) -> Drain<'_, DamageType> {
    self.0.drain(..)
  }
}
