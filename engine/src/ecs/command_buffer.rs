use std::{alloc::{alloc, dealloc, Layout}, any::TypeId, ops::Range, ptr::{self,NonNull}};

use crate::component_lib::{AutoAttackMesh, AutoAttackScript, MissleSpeed, Owner, Position, PreviousPosition, Target, Velocity};

use super::{bundle::{Bundle, TypeInfo}, query::ComponentRef};

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
struct CommandBuffer{
  commands: Vec<Command>,
  component_data: Vec<ComponentInfo>,
  storage:NonNull<u8>,
  layout: Layout,
  cursor: usize,
  ids: Vec<TypeId>
}



impl CommandBuffer {
  pub fn new()->Self{
    CommandBuffer::default()
  }

  //https://chat.openai.com/c/094a8062-9b95-41a2-b680-719d80f89d3f
  //https://docs.rs/hecs/latest/src/hecs/command_buffer.rs.html#33-40
  //idk how long this link lasts but basically I asked GPT to explain the grow/insert_inner functions in Hec's command buffer

  //Questions:
  //do the values on self need to also be params?
  //what is NonNull
  //what does alloc/dealloc do? Hec seems to use its own version so check the github
  //What is a layout
  //what does copy nonoverlapping do
  //what is the cursor?
  //should I use 32 in case I ever want to chuck something at a 32 bit system?
  
  unsafe fn grow(
    min_size:usize,
    cursor:usize,
    align:usize,
    storage:NonNull<u8>
  ) -> (NonNull<u8>, Layout) {
    //Calculate the size and Layout of the new chunk of data by growing the memory size by a power of two
    let size = min_size.checked_next_power_of_two().unwrap_or(0).max(64);
    let layout = Layout::from_size_align(size, align).unwrap();
    
    //Allocate a region in memory capable of holding data corresponding to layout and return a pointer to that location
    let new_storage = NonNull::new_unchecked(alloc(layout));
    
    //Copy the data from the old storage to the new storage
    ptr::copy_nonoverlapping(storage.as_ptr(), new_storage.as_ptr(), cursor);
    
    (new_storage, layout)
  }

  //Why do I only dallocate the storage if the size is not zero?
  //What is the bit at the end beginning with addr doing? I need to document it
  //Why does count in copy = layout.size
  unsafe fn add_inner(&mut self, ptr: *mut u8, type_info: TypeInfo){
    //Get the amount of padding necesary for the new data chunk to start on a power of two of the alignment
    let start = align(self.cursor, type_info.layout().align());
    //The end of the data chunk after alignment
    let end = start + type_info.layout().size();

    //Check if adding the component would cause it to exceed the currently allocated memory
    //Occurs if the end of the data chunk exceeds the size of the struct 
    //Occurs if the alignment of the new data is bigger than the alignment of the CommandBuffer
    if end > self.layout.size() || type_info.layout().align() > self.layout.align() {
      //Calculate the new alignment of the CommandBuffer
      //New alignment is whichever is bigger between the new data's alignment and the current CommandBuffer's alignment
      let new_align = self.layout.align().max(type_info.layout().align());
      
      //Allocate memory for the struct, copy data to the new memory, get a pointer to the new data and the new alignment of the CommandBuffer
      let (new_storage,new_layout) = Self::grow(end, self.cursor, new_align, self.storage);

      if self.layout.size() != 0 {
        dealloc(self.storage.as_ptr(), self.layout)
      }

      //Update the CommandBuffer with the pointer to the data and the new layout
      self.storage = new_storage;
      self.layout = new_layout;
    }
    
    //Calculate the address in memory where the new chunk should be copied. 
    let addr = self.storage.as_ptr().add(start);
    //Copy the data
    ptr::copy_nonoverlapping(ptr, addr, type_info.layout().size());
    
    //Append info about the newly added components to the Command buffers component data
    let component_info = ComponentInfo{type_info, start};
    self.component_data.push(component_info);
  }

  ///Queues a new entity to be added to world with the provided components.
  pub fn create_entity(&mut self, components:impl Bundle){
    //takes the component data
    //move ownership of the data into the Buffer from the function
    // let first = self.component_data.len();

    // unsafe{
    //   components.put(|ptr, ty|self.add_inner(ptr, ty))
    // }

    // self.commands.push(Command::Spawn(
    //   EntityIndex{
    //     entity_id: None,
    //     components:first..self.component_data.len()
    //   }
    // ));
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

///Utility function which ensures the result is the smallest multiple of alignment greater than or equal to the cursor's position.
/// Useful for padding. Can be used to ensure data starts at an address which is a multiple of the given alignment.  
fn align(cursor:usize, alignment:usize) -> usize {
  debug_assert!(alignment.is_power_of_two());
  (cursor + alignment - 1) & (!alignment + 1)
}

//using the type info and offset the component can be copied to the appropriate location. use the type id to place it and the size to figure out how many slots to copy.

pub struct ComponentInfo{
  type_info: TypeInfo,
  //indicates the starting location of the component in memory
  start:usize
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
      component_data: Vec::new(),
      storage: NonNull::dangling(),
      layout: Layout::from_size_align(0, 8).unwrap(),
      cursor: 0,
      ids: Vec::new(),
    }
  }
}
//Do you use the drop trait to run the buffered commands?