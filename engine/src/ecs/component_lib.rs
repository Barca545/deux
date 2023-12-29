use gl::Gl;
use serde::{Deserialize, Serialize};

use crate::{math::math::Vec3, physics::AABB3D, view::{render_gl::Vertex, Mesh}};
//unsure if this is where I should store stuff like movespeed
//why does making both dyn Any cause an issue? Says the size for both must be
// known at compile time but I thought that defeated the point of any?

///Represents units the player can control.
#[derive(Debug, Clone, Copy)]
pub struct Controllable;

#[derive(Debug, Clone, Copy)]
pub struct Health(i32);

//I think I want to separate these into two components
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

#[derive(Debug, Default, Clone, Copy)]
pub struct Velocity(pub Vec3);

impl Velocity {
  pub fn new(position:&Vec3, destination:&Vec3, speed:&f32) -> Self {
    let velocity:Vec3 = (destination - position).normalize().scale(*speed);
    Velocity(velocity)
  }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Speed(pub f32);

#[derive(Debug, Clone, Copy)]
///3D AABB to be used for unit selection.
pub struct SelectionRadius(pub AABB3D);

impl SelectionRadius {
  pub fn new(position:Vec3, height:f32, radius:f32) -> Self {
    let aabb3d = AABB3D::new(position, height, radius);

    SelectionRadius(aabb3d)
  }
}

#[derive(Debug, Clone, Copy)]
///Radius for edge-to-edge gameplay logic.
pub struct GameplayRadius(pub f32);

///Radius for unit collision and pathing logic.
#[derive(Debug, Clone, Copy)]
pub struct PathingRadius(pub f32);

//Can use the following two to construct a ward entity.
//Duration can be reused for other stuff too.
pub struct VisionRange(i32);
pub struct Duration(f64);

//these probably need to hold a duration so the can be timed
pub enum MovementState {
  DASHING,
  WALKING
}

pub enum CrowdControlState {
  STUNNED(usize),
  SLOWED(usize),
  AIRBORNE(usize)
}

pub type CrowdControlList = Vec<CrowdControlState>;

//Combat
pub struct Target(pub Option<usize>);

#[derive(Debug, Clone, Copy)]
pub struct MissleSpeed(pub f32);

#[derive(Debug, Clone, Copy, Default)]
pub struct AutoAttack;

#[derive(Debug, Clone, Copy)]
//use the seconds thing imported from the timer mod
pub struct AutoAttackCooldown{
  pub duration:f64,
  pub remaining:f64
}

impl AutoAttackCooldown{
  pub fn new(duration:f64,remaining:f64) -> Self {
    AutoAttackCooldown{duration,remaining}
  }
}

//Identification
#[derive(PartialEq)]
pub enum Team{
  BLUE,
  RED,
  NEUTRAL
}


//rendering
pub struct SkinnedMesh(pub Mesh);
impl SkinnedMesh{
  pub fn new(gl: &Gl, vertices: Vec<Vertex>, indices: Vec<u32>, texture_name: &str) -> Self{
    SkinnedMesh(Mesh::new(gl, vertices, indices, texture_name))
  }
}

impl From<AutoAttackMesh> for SkinnedMesh{
  fn from(value: AutoAttackMesh) -> Self {
    let mesh = value.0;
    SkinnedMesh(mesh)
  }
}

pub struct StaticMesh(pub Mesh);
impl StaticMesh{
  pub fn new(gl: &Gl, vertices: Vec<Vertex>, indices: Vec<u32>, texture_name: &str) -> Self{
    StaticMesh(Mesh::new(gl, vertices, indices, texture_name))
  }
}

#[derive(Debug, Clone)]
pub struct AutoAttackMesh(pub Mesh);
impl AutoAttackMesh{
  pub fn new(gl: &Gl, vertices: Vec<Vertex>, indices: Vec<u32>, texture_name: &str) -> Self{
    AutoAttackMesh(Mesh::new(gl, vertices, indices, texture_name))
  }
}
