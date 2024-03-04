use serde::{Deserialize, Serialize};

// Refactor:
// -Probably need to remove the Killed and Colliding components

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
///Component which identifies an entity  as an auto attack.
pub struct AutoAttack;

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
///Component which identifies an entity as a player. 1-10 allowed as values.
pub struct Player(pub u32);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
///Component which flags an entity the player can control.
pub struct Controllable;

#[derive(Debug, Default, Clone, Copy)]
///Component containing the id of an entity's target.
pub struct Target(pub Option<usize>);

#[derive(Debug, Clone, Copy)]
///Component containing the id of an entity's owner.
pub struct Owner(pub usize);

#[derive(Debug, Clone, Copy)]
///Component containing the id of an entity the holder has killed.
pub struct Killed(pub usize);

///Component indicating an entity is dead.
#[derive(Debug, Clone, Copy)]
pub struct Dead;

#[derive(PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
///Component containing an entity's team identification.
pub enum Team {
  Blue,
  Red,
  Neutral,
}
