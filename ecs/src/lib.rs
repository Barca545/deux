pub mod custom_errors;
pub mod entities;
pub mod resources;

use entities::{entities::Entities, query::Query};
use eyre::Result;
use resources::Resource;
use sdl2::rect::{Point, Rect};
use std::{any::Any, ffi::CString};

/*
Questions
- what is a Refcell and what is RC
- what is + 'static
- what is an Option
- what is an iter/iter_mut
- for_each syntax
- what is a closure or closer
- what is self vs Self
- what does move out mean
- what does ok_or/ok_or_else
- what does unwrap() do
- what does derive default mean
- what is turbofish syntax
- what does default do
- what does a Deref do
- what is downcasting
- what does Some do
- how does |= work
- difference between | and & in rust
- what is a BitXor (^) in rust -> Compares the binary representations of two numbers and adds them when the bits for a given place are not equal so 101 & 010 = 111 and 110 ^ 100 = 010
- as_ref vs &
- are the ids the same as the indices in my code?
- integration vs unit test?
- what is a collection?
*/

//confirm my documentation for the entities portion is correct

//does it make sense to make a special thing for champions? I do not think so
#[derive(Default, Debug)]
pub struct World {
  resources:Resource,
  entities:Entities
}

impl World {
  /**
  Generates a new world with default settings.
  ```
  use ecs::World;
  let mut world = World::new();
  ```
  */
  pub fn new() -> Self {
    Self::default()
  }

  /**
  Adds a resource to the world. Use with `from_user_defined_data`, `from_relative_exe_path`
  ```
  use ecs::World;
  let mut world = World::new();
  world.add_resource().from_user_defined_data(10_u32);
  world.add_resource().from_relative_exe_path("dummy.path");
  ```
  */
  pub fn add_resource(&mut self) -> &mut Resource {
    &mut self.resources
  }

  /**
  Query a resource by type and get a mutable reference. The type of the resource must be added.
  ```
  use ecs::World;
  let mut world = World::new();
  world.add_resource(10_u32);
  let resource = world.mut_get_resource::<u32>().unwrap();
  {
      let resource = world.mut_get_resource::<u32>().unwrap();
      *resource +=1
  }
  let resource = world.immut_get_resource::<u32>().unwrap();
  assert_eq!(*resource,11)
  ```
  */
  pub fn mut_get_resource<T:Any>(&mut self) -> Option<&mut T> {
    self.resources.get_mut::<T>()
  }

  /**
  Query a resource by type and get an immutable reference. The type of the resource must be added.
  ```
  use ecs::World;

  let mut world = World::new();
  world.add_resource(10_u32);
  let resource = world.immut_get_resource::<u32>().unwrap();
  assert_eq!(*resource,10)
  ```
  */
  pub fn immut_get_resource<T:Any>(&self) -> Option<&T> {
    self.resources.get_ref::<T>()
  }

  pub fn load_resource_from_cstring(&self, resource_name:&str) -> Result<CString> {
    self.resources.load_resource_from_cstring(resource_name)
  }

  /**
  Takes in a type and removes the resource from the World. Does not care if the resource exists.
  ```
  use ecs::World;
  use std::any::TypeId;

  let mut world = World::new();
  world.add_resource(10_u32);
  world.remove_resource::<u32>();
  ```
  */
  pub fn remove_resource<T:Any>(&mut self) {
    self.resources.remove::<T>()
  }

  ///Tells Entities that entities inside will have a component of type T.
  pub fn register_component<T:Any + 'static>(&mut self) -> &mut Entities {
    self.entities.register_component::<T>()
  }

  ///Creates a new entity adds it to the entities list. Iterates over the
  /// registered components and initializes them with 'None'. Sets the bitmap
  /// for the entity to 0 indicating it has no components associated with it.
  /// Use with `.with_component()` to create an entity with components.
  pub fn create_entity(&mut self) -> &mut Entities {
    self.entities.create_entity()
  }

  pub fn query(&self) -> Query {
    Query::new(&self.entities)
  }

  ///"Deletes" a component from an entity. Note: The component is not deleted
  /// it is simply removed from the entit's bitmap.
  pub fn delete_component_by_entity_id<T:Any>(&mut self, index:usize) -> Result<()> {
    self.entities.delete_component_by_entity_id::<T>(index)
  }

  ///Takes in data as a new component
  pub fn add_component_to_entity_by_id(&mut self, data:impl Any, index:usize) -> Result<()> {
    self.entities.add_component_by_entity_id(data, index)
  }

  ///Deletes an entity from the entities list matching the index.
  /// Leaves the slot open -- the next entity added will overwrite the emptied
  /// slot.
  pub fn delete_entity_by_id(&mut self, index:usize) -> Result<()> {
    self.entities.delete_entity_by_id(index)
  }

  //obv these can't all spawn in the same place. These values will need to be fed
  // in from somewhere (probably scrips for stats) Kitten says to make sure this
  // stays in memory instead of getting cleared up by garbage collection.
  // this maybe should be a part of a system rather than the world?
  pub fn create_player_champion(&mut self) -> Result<()> {
    self.register_component::<Sprite>();
    self.register_component::<Position>();
    self.register_component::<Speed>();
    self.register_component::<Velocity>();
    self.register_component::<Target>();

    self
      .create_entity()
      .with_component(Sprite(Rect::new(0, 0, 26, 36)))?
      .with_component(Position(Point::new(0, 0)))?
      .with_component(Speed(2))?
      .with_component(Velocity(Point::new(0, 0)))?
      .with_component(Target(Point::new(0, 0)))?;
    Ok(())
  }
}

//might make sense to make a separate components module
//Resources
pub struct ScreenDimensions {
  pub height:i32,
  pub width:i32,
  pub aspect:f32
}
impl ScreenDimensions {
  pub fn new(height:i32, width:i32) -> Self {
    let aspect = width as f32 / height as f32;
    ScreenDimensions { height, width, aspect }
  }
}
//components
pub struct Sprite(pub Rect);
pub struct Position(pub Point);
pub struct Speed(pub u32);
pub struct Velocity(pub Point);
pub struct Target(pub Point);

#[cfg(test)]
mod tests {}
