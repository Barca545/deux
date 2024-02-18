use serde::{Deserialize, Serialize};
use crate::math::Vec3;

#[derive(Debug, Clone, Copy, Default)]
///Component holding the position of the entity during the current game logic tick.
pub struct Position(pub Vec3);

#[derive(Debug, Clone, Copy, Default)]
///Component holding the position the entity occupied at the end of the previous game logic tick.
pub struct PreviousPosition(pub Vec3);
impl From<Position> for PreviousPosition {
  fn from(value: Position) -> Self {
    PreviousPosition(value.0)
  }
}

#[derive(Debug, Clone, Copy)]
///Component holding the position an entity is moving towards.
pub struct Destination(pub Vec3);
impl From<Position> for Destination{
  fn from(value: Position) -> Self {
    Destination(value.0)
  }
}

impl From<[f32;3]> for Destination{
  fn from(value: [f32;3]) -> Self {
    Destination(Vec3::from(value))
  }
}

#[derive(Debug, Default, Clone, Copy)]
///Component holding the velocity of an entity towards its `Destination`.
pub struct Velocity(pub Vec3);
impl Velocity {
  pub fn new(position:&Position, destination:&Destination, speed:&f32) -> Self {
    let velocity:Vec3 = (destination.0 - position.0).normalize().scale(*speed);
    Velocity(velocity)
  }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
///Component holding a unit entity's speed.
pub struct UnitSpeed(pub f32);

#[derive(Debug, Clone, Copy)]
///Component marker for entities undergoing a collision.
pub struct Colliding;

///Component containing a `Vec<Destination>` used for storing an entity's path. 
pub struct Path{pub nodes: Vec<Destination>}

impl Path{
  pub fn new() -> Self {
    Path { nodes: Vec::default() }
  }

  ///Returns an option containing the next Destination in the Path. 
  /// If the final destination has been reached, returns `None`.
  pub fn next(&mut self) -> Option<Destination> {
    self.nodes.pop()
  }

  pub fn push(&mut self,node:Destination){
    self.nodes.push(node)
  }

  pub fn len(&self) -> usize {
    self.nodes.len()
  }
}