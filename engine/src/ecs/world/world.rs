use crate::{
  ecs::{
    bundle::{Bundle, TypeInfo},
    entities::Entities,
    query::Query,
    resources::Resource,
  },
  errors::EcsErrors,
};

use eyre::Result;
use std::{
  any::{Any, TypeId},
  cell::{Ref, RefMut},
};

//confirm my documentation for the entities portion is correct

//Refactor
// -Set it up so gettng the resources can be an ? and not an unwrap
// -Change naming patter from immut/mut_get to just get/get_mut
// -Create a spawn function. Might require implementing a real command buffer?
// -Investigate archetype based ECS systems

#[derive(Default, Debug)]
pub struct World {
  resources: Resource,
  entities: Entities,
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
  world.add_resource(10_u32);
  ```
  */
  pub fn add_resource(&mut self, data: impl Any) -> &mut Self {
    self.resources.add_resource(data);
    self
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
  pub fn get_resource_mut<T: Any>(&self) -> Result<RefMut<T>> {
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
  pub fn get_resource<T: Any>(&self) -> Result<Ref<T>> {
    self.resources.get_ref::<T>()
  }

  ///Query immutably for the specified component data from the entity whose ID matches the given index.
  pub fn get_component<T: Any>(&self, index: usize) -> Result<Ref<T>> {
    let ty = TypeInfo::of::<T>();

    let components = self.entities.components.get(&ty.id()).ok_or(EcsErrors::ComponentNotRegistered)?;

    let borrowed_component = components[index]
      .as_ref()
      .ok_or(EcsErrors::CreateComponentNeverCalled {
        component: ty.type_name().to_string(),
      })?
      .borrow();

    Ok(Ref::map(borrowed_component, |any| any.downcast_ref::<T>().unwrap()))
  }

  ///Query mutably for the specified component data from the entity whose ID matches the given index.
  pub fn get_component_mut<T: Any>(&self, index: usize) -> Result<RefMut<T>> {
    let typid = TypeId::of::<T>();

    let components = self.entities.components.get(&typid).ok_or(EcsErrors::ComponentNotRegistered)?;

    let borrowed_component = components[index].as_ref().ok_or(EcsErrors::ComponentDataDoesNotExist)?.borrow_mut();

    Ok(RefMut::map(borrowed_component, |any| any.downcast_mut::<T>().unwrap()))
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
  pub fn remove_resource<T: Any>(&mut self) {
    self.resources.remove::<T>()
  }

  ///Updates the Entities to include components of type T.
  pub fn register_component<T: Any + 'static>(&mut self) -> &mut Entities {
    self.entities.register_component::<T>()
  }

  ///Creates a new entity adds it to the entities list. Iterates over the
  /// registered components and initializes them with 'None'. Sets the bitmap
  /// for the entity to 0 indicating it has no components associated with it.
  /// Use with `.with_component()` to create an entity with components.
  pub fn create_entity(&mut self) -> &mut Entities {
    self.entities.create_entity()
  }

  ///Reserves an entity id. See: `create_entity`.
  pub fn reserve_entity(&mut self) -> usize {
    self.entities.reserve_entity()
  }

  ///Creates a query to access entities in the `World` instance.
  pub fn query(&self) -> Query {
    Query::new(&self.entities)
  }

  ///Updates the bitmap of the entity matching the provided ID to indicate it does not contain the component type.
  pub fn remove_component<T: Any>(&mut self, index: usize) -> Result<()> {
    self.entities.delete_component_by_entity_id::<T>(index)
  }

  pub(crate) fn remove_component_by_typeinfo(&mut self, index: usize, ty: TypeInfo) -> Result<()> {
    self.entities.delete_component_by_type_info(index, ty)
  }

  ///Adds a component to the entity matching the provided ID.
  pub fn add_component(&mut self, entity: usize, data: impl Any) -> Result<()> {
    self.entities.add_component_by_entity_id(entity, data)
  }

  ///Adds a bundle of components to an entity.
  /// Does not error if component is unregistered but operation will fail.
  pub fn add_components(&mut self, index: usize, components: impl Bundle) {
    self.entities.add_components(index, components)
  }

  ///Deletes an entity from the entities list matching the index.
  /// Leaves the slot open -- the next entity added will overwrite the emptied slot.
  pub fn delete_entity(&mut self, index: usize) -> Result<()> {
    self.entities.delete_entity(index)
  }
}

#[cfg(test)]
mod tests {}
