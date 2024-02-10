use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
///Component containing the speed of a missle entity.
pub struct MissleSpeed(pub f32);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
///Component containing the attack damage of an entity.
pub struct AttackDamage(pub i32);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Armor(pub i32);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Health{
  pub max:i32,
  pub remaining:i32
}
impl Health{
  pub fn new(max:i32) -> Self {
    Health { 
      max, 
      remaining: max
    }
  }
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
///Component which tracks the kills, deaths, and assits of an entity.
pub struct KDA{
  kills:u32,
  deaths:u32,
  assists:u32
}
impl KDA { 
  ///Increments the tracked kills by 1.
  pub fn kill(&mut self, number:u32){
    self.kills += number;
  }
  
  ///Increments the tracked deaths by 1.
  pub fn death(&mut self, number:u32){
    self.deaths += number;
  }

  ///Increments the tracked assists by 1.
  pub fn assist(&mut self, number:u32){
    self.assists += number;
  }
}