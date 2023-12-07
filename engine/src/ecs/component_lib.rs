use std::any::Any;
use glm::Vec3;

use crate::view::render_gl::UncoloredTexturedVertex;
//unsure if this is where I should store stuff like movespeed
//why does making both dyn Any cause an issue? Says the size for both must be known at compile time but I thought that defeated the point of any?

pub struct Health(i64);
pub struct Position(pub Vec3);
pub struct Model(pub Vec<UncoloredTexturedVertex>);
pub struct Hitbox{
  center: Box<dyn Any>, //probably should just use vertex's for this or something
  bounding_box: Box<dyn Any>
}

//Can use the following two to construct a ward entity. 
//Duration can be reused for other stuff too.
pub struct VisionRange(i32);
pub struct Duration(f64);

pub enum MovementState {
  DASHING,
  WALKING
} 

pub type EntityId = i32; //probably can't be a number. This is a placeholder.

pub enum CrowdControlState {
  STUNNED(EntityId),
  SLOWED(EntityId),
  AIRBORNE(EntityId)
} 

pub type  CrowdControlList = Vec<CrowdControlState>;

