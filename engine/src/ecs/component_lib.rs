use gl::Gl;
use serde::{Deserialize, Serialize};

use crate::{math::math::Vec3, physics::AABB3D, view::{render_gl::Vertex, Mesh}};
//unsure if this is where I should store stuff like movespeed
//why does making both dyn Any cause an issue? Says the size for both must be
// known at compile time but I thought that defeated the point of any?

///Represents units the player can control.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Controllable;

//I think I want to separate these into two components
#[derive(Debug, Clone, Copy, Default)]
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

impl From<Vec3> for Destination{
  fn from(value: Vec3) -> Self {
    Destination(value)
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
///Radius for edge-to-edge gameplay logic.
pub struct GameplayRadius(pub f32);

///Radius for unit collision and pathing logic.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PathingRadius(pub f32);

//Can use the following two to construct a ward entity.
//Duration can be reused for other stuff too.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct VisionRange(i32);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Duration(f64);

//Player State
//these probably need to hold a duration so the can be timed
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MovementState {
  DASHING,
  WALKING
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CrowdControlState {
  STUNNED(usize),
  SLOWED(usize),
  AIRBORNE(usize)
}

pub type CrowdControlList = Vec<CrowdControlState>;

//Combat
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Target(pub Option<usize>);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Owner{
  pub id:usize
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MissleSpeed(pub f32);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
//use the seconds thing imported from the timer mod
pub struct AutoAttackCooldown{
  //this type will be reused and probably should be its own struct
  pub duration:f64,
  pub remaining:f64
}

impl AutoAttackCooldown{
  pub fn new(duration:f64,remaining:f64) -> Self {
    AutoAttackCooldown{duration,remaining}
  }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AttackDamage(pub i32);

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

//Level and exp
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Exp(pub u32);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Level(pub u32);

impl Default for Level{
  fn default() -> Self {
    Self(1)
  }
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Gold(pub i32);

//the different events should probably get a timestamp
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct KDA{
  kills:u32,
  deaths:u32,
  assists:u32
}

impl KDA { 
  pub fn kill(&mut self, number:u32){
    self.kills += number;
  }
  
  pub fn death(&mut self, number:u32){
    self.deaths += number;
  }

  pub fn assist(&mut self, number:u32){
    self.assists += number;
  }
}

//Identification
#[derive(PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Team{
  BLUE,
  RED,
  NEUTRAL
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct AutoAttack;

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
///Flags an entity as a player. 1-10 allowed as values.
pub struct Player(pub u32);

//rendering
pub struct SkinnedMesh{
  pub mesh:Mesh,
  pub scale_factor:f32
}

impl SkinnedMesh{
  pub fn new(gl: &Gl, vertices: Vec<Vertex>, indices: Vec<u32>, texture_name: &str, scale_factor:f32) -> Self{
    SkinnedMesh{
      mesh:Mesh::new(gl, vertices, indices, texture_name).to_owned(),
      scale_factor
    }
  }
}

impl From<AutoAttackMesh> for SkinnedMesh{
  fn from(value: AutoAttackMesh) -> Self {
    let mesh = value.mesh;
    let scale_factor = value.scale_factor;
    SkinnedMesh{
      mesh,
      scale_factor
    }
  }
}

#[derive(Debug, Clone)]
pub struct AutoAttackMesh{
  pub mesh:Mesh,
  pub scale_factor:f32
}
impl AutoAttackMesh{
  pub fn new(gl: &Gl, vertices: Vec<Vertex>, indices: Vec<u32>, texture_name: &str, scale_factor:f32) -> Self{
    AutoAttackMesh{
      mesh:Mesh::new(gl, vertices, indices, texture_name).to_owned(),
      scale_factor
    }
  }
}

pub struct StaticMesh(pub Mesh);
impl StaticMesh{
  pub fn new(gl: &Gl, vertices: Vec<Vertex>, indices: Vec<u32>, texture_name: &str) -> Self{
    StaticMesh(Mesh::new(gl, vertices, indices, texture_name))
  }
}
