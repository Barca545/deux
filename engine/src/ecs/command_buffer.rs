use crate::ecs::{component_lib::{AutoAttackMesh, MissleSpeed, Owner, Position, AutoAttackScript, Target, Velocity}, query::ComponentRef};

//Not currently a CommandBuffer. Can be expanded into a CommandBuffer like Hecs uses when I eventually need to replicated this functionality
//https://docs.rs/hecs/latest/src/hecs/command_buffer.rs.html#33-40
#[derive(Debug, Clone)]
pub struct AutoAttackSpawner{
  pub indices:Vec<usize>,
  pub positions:Vec<Position>,
  pub missle_speeds:Vec<MissleSpeed>,
  pub velocities:Vec<Velocity>,
  pub meshes:Vec<AutoAttackMesh>,
  pub owners:Vec<Owner>,
  pub targets:Vec<Target>,
  pub scripts: Vec<ComponentRef<AutoAttackScript>>,
}

impl Default for AutoAttackSpawner{
  fn default() -> Self {
    Self { 
      indices: Default::default(),
      positions: Default::default(),
      missle_speeds: Default::default(),
      velocities: Default::default(),
      meshes: Default::default(),
      owners: Default::default(),
      targets: Default::default(),
      scripts: Default::default(),
    }
  }
}

impl AutoAttackSpawner{
  pub fn add(
    &mut self, 
    position:Position,
    missle_speed:MissleSpeed,
    velocity:Velocity,
    mesh:AutoAttackMesh,
    owner:Owner,
    target:Target,
    script:ComponentRef<AutoAttackScript>
  )
  {
    let index = self.indices.len();
    self.indices.push(index);
    self.positions.push(position);
    self.missle_speeds.push(missle_speed);
    self.velocities.push(velocity);
    self.meshes.push(mesh);
    self.owners.push(owner);
    self.targets.push(target);
    self.scripts.push(script);
  }
}

//Do you use the drop trait to run the buffered commands?