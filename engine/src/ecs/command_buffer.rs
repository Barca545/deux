use std::{any::Any, ops::Range};

use crate::component_lib::{AutoAttackMesh, AutoAttackScript, MissleSpeed, Owner, Position, PreviousPosition, Target, Velocity};

use super::{bundle::Bundle, query::ComponentRef};

//Not currently a CommandBuffer. Can be expanded into a CommandBuffer like Hecs uses when I eventually need to replicated this functionality
//https://docs.rs/hecs/latest/src/hecs/command_buffer.rs.html#33-40
#[derive(Debug, Clone, Default)]
pub struct AutoAttackSpawner{
  pub indices:Vec<usize>,
  pub positions:Vec<Position>,
  pub previous_positions:Vec<PreviousPosition>,
  pub missle_speeds:Vec<MissleSpeed>,
  pub velocities:Vec<Velocity>,
  pub meshes:Vec<AutoAttackMesh>,
  pub owners:Vec<Owner>,
  pub targets:Vec<Target>,
  pub scripts: Vec<ComponentRef<AutoAttackScript>>,
}

impl AutoAttackSpawner{
  pub fn add(
    &mut self, 
    position:Position,
    previous_position:PreviousPosition,
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
    self.previous_positions.push(previous_position);
    self.missle_speeds.push(missle_speed);
    self.velocities.push(velocity);
    self.meshes.push(mesh);
    self.owners.push(owner);
    self.targets.push(target);
    self.scripts.push(script);
  }
}


// Refactor
// -Could add_inner be made faster by assuming the new alignment is always the ty.alignment or something if I reconfiger the conditionals?
// -Could grow be a method instead of an associated function?
// -Add a method to world/entities to reserve an entity id
// -Move the unsafe stuff to Becs (Brainy ECS) and retry to implement it once I am more skilled

struct CommandBuffer{
  commands: Vec<Command>,
  components: Vec<Box<dyn Any>>
}

impl CommandBuffer {
  pub fn new()->Self{
    CommandBuffer::default()
  }

  ///Queues a new entity to be added to world with the provided components.
  pub fn create_entity(&mut self, components:impl Bundle){
  }

  pub fn delete_entity(&mut self, id:EntityId){}

  pub fn add_components(&mut self, id:EntityId, components:impl Bundle){
    // let start = self.component_data.len()-1;
    // let stop = components.len();
  }

  pub fn remove_components<B:Bundle>(&mut self,){
    //is it possible to iterate over the components in the turbofish
  }
}

//this type should actually be used throughout where relevant
type EntityId = usize;

pub struct EntityIndex{
  //Target entity for an add command
  entity_id:Option<EntityId>,
  //Component belonging to the entity in the CommandBuffer's component data vec
  components: Range<usize>
}

pub enum Command {
  Spawn(EntityIndex),
  Despawn(EntityId),
  Insert(EntityIndex),
  Remove(EntityIndex),
}

impl Default for CommandBuffer{
  fn default() -> Self {
    Self {
      commands: Vec::new(),
      components: Vec::new()
    }
  }
}
//Do you use the drop trait to run the buffered commands?