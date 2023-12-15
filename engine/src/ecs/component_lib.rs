use crate::{math::math::Vec3, view::render_gl::Vertex, physics::AABB3D};
//unsure if this is where I should store stuff like movespeed
//why does making both dyn Any cause an issue? Says the size for both must be
// known at compile time but I thought that defeated the point of any?

///Represents units the player can control.
#[derive(Debug, Clone, Copy)]
pub struct Controllable;

#[derive(Debug, Clone, Copy)]
pub struct Health(i64);

#[derive(Debug, Clone, Copy)]
pub struct Position {
  pub tick_start:Vec3,
  pub tick_end:Vec3
}

impl Position {
  pub fn new(tick_start:Vec3, tick_end:Vec3) -> Self {
    Position { tick_start, tick_end }
  }
}

#[derive(Debug, Clone, Copy)]
pub struct Destination(pub Vec3);
impl Destination {
  pub fn new(x:f32, y:f32, z:f32) -> Self {
    Destination(Vec3::new(x, y, z))
  }
}

#[derive(Debug, Clone, Copy)]
pub struct Velocity(pub Vec3);
impl Velocity {
  pub fn new(position:&Position, destination:&Destination, speed:&Speed) -> Self {
    let velocity:Vec3 = (destination.0 - position.tick_end).normalize().scale(speed.0);
    Velocity(velocity)
  }
}

#[derive(Debug, Clone, Copy)]
pub struct Speed(pub f32);

// #[derive(Debug,Clone,Copy)]
pub struct Model(pub Vec<Vertex>);

pub struct GroundModel(pub Vec<Vertex>);

#[derive(Debug,Clone,Copy)]
//this should be the inner and outer bounding box
pub struct Hitbox {
  pub inner:AABB3D,
  pub outer:AABB3D
}

impl Hitbox {
  pub fn new(position:Vec3, height:f32, inner_radius:f32,outer_radius:f32) -> Self {
    let inner = AABB3D::new(position, height, inner_radius);
    let outer = AABB3D::new(position, height, outer_radius);

    Hitbox{
      inner,
      outer
    }
  }
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

pub type CrowdControlList = Vec<CrowdControlState>;

#[derive(Debug,Clone,Copy)]
pub struct TestComponent<'a>(pub &'a str);
