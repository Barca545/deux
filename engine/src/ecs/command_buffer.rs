use super::{
  bundle::{Bundle, TypeInfo},
  World,
};
use std::any::Any;

// Refactor
// -Could add_inner be made faster by assuming the new alignment is always the ty.alignment or something if I reconfiger the conditionals?
// -Could grow be a method instead of an associated function?
// -Add a method to world/entities to reserve an entity id
// -Move the unsafe stuff to Becs (Brainy ECS) and retry to implement it once I am more skilled
// -Not currently a CommandBuffer. Can be expanded into a CommandBuffer like Hecs uses when I eventually need to replicated this functionality
// -https://docs.rs/hecs/latest/src/hecs/command_buffer.rs.html#33-40

pub struct CommandBuffer {
  commands: Vec<Command>,
  components: Vec<Box<dyn Any>>,
}

impl CommandBuffer {
  pub fn new() -> Self {
    CommandBuffer::default()
  }

  ///Queues a new entity to be added to world with the provided components.
  pub fn create_entity(&mut self, components: impl Bundle) {}

  pub fn delete_entity(&mut self, id: EntityId) {}

  pub fn add_component<T: Any>(&mut self, id: EntityId, data: T) {
    let ty = TypeInfo::of::<T>();
    let command = Command::InsertOne(id, ty, Box::new(data));
    self.commands.push(command);
  }

  pub fn add_components(&mut self, id: EntityId, components: impl Bundle) {
    // let start = self.component_data.len()-1;
    // let stop = components.len();
  }

  pub fn remove_components<B: Bundle>(&mut self) {
    //is it possible to iterate over the components in the turbofish
  }

  pub fn remove_component<T: Any>(&mut self, entity: EntityId) {
    let command = Command::Remove(entity, TypeInfo::of::<T>());
    self.commands.push(command);
  }

  pub fn run(&mut self, world: &mut World) {
    for command in &self.commands {
      match command {
        Command::Remove(entity, ty) => {
          world.remove_component_by_typeinfo(*entity, *ty).unwrap();
        }
        _ => {}
      }
    }
  }
}

//this type should actually be used throughout where relevant
type EntityId = usize;

#[derive(Debug, Clone, Copy)]
pub struct EntityIndex {
  //Target entity for an add command
  entity_id: Option<EntityId>,
  //Component belonging to the entity in the CommandBuffer's component data vec
  // components: Range<usize>
}

#[derive(Debug)]
pub enum Command {
  Spawn(EntityIndex),
  Despawn(EntityId),
  InsertOne(EntityId, TypeInfo, Box<dyn Any>),
  Remove(EntityId, TypeInfo),
}

impl Default for CommandBuffer {
  fn default() -> Self {
    Self {
      commands: Vec::new(),
      components: Vec::new(),
    }
  }
}
//Do you use the drop trait to run the buffered commands?
