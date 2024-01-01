use eyre::Result;
use std::{
  any::{Any, TypeId},
  cell::RefCell,
  collections::HashMap,
  rc::Rc
};

//why does this require pub mod be declared in both main and in lib
use crate::errors::EcsErrors;

//From YT: https://www.youtube.com/watch?v=_9fAYLWSBVE&list=PLrmY5pVcnuE_SQSzGPWUJrf9Yo-YNeBYs&index=42
//I think a better approach would be to have a vec of free spots/indexes and
// that we keep adding to whenever we delete an entity, then when we add an
// entity, we look at the last available index in the free spots vec
// and we insert the components at that index, then we simply just .pop() on the
// vec of free indexes, this should be faster for large amounts of entities This
// will be used a lot so making it as fast as possible is good

pub type Component = Rc<RefCell<dyn Any>>;
pub type Components = HashMap<TypeId, Vec<Option<Component>>>;

//so you can search for classes of entities because they will all have the same
// number for their bitmap

//use another bitmask for specific id?
#[derive(Debug, Default)]
pub struct Entities {
  //what is this type
  pub components:Components,
  //using u32 limits us to 32 components per entity
  pub bitmasks:HashMap<TypeId, u128>,
  //this was not pub in the tutorial
  //contains the bitmaps for the registered components
  pub map:Vec<u128>,
  inserting_into_index:usize
}

impl Entities {
  pub fn register_component<T:Any + 'static>(&mut self) -> &mut Self {
    let typeid = TypeId::of::<T>();
    self.components.insert(typeid, vec![]);
    self.bitmasks.insert(typeid, 1 << self.bitmasks.len());
    self
  }

  pub fn create_entity(&mut self) -> &mut Self {
    if let Some((index, _)) = self.map.iter().enumerate().find(|(_index, mask)| **mask == 0) {
      self.inserting_into_index = index;
    } else {
      self.components.iter_mut().for_each(|(_key, components)| components.push(None));
      self.map.push(0);
      self.inserting_into_index = self.map.len() - 1;
    }
    self
  }

  ///Used with `create_entity` to assign components and their initial values to
  /// the entity being created. Updates the entity's bitmap to indicate which
  /// components they contain.
  pub fn with_component(&mut self, data:impl Any) -> Result<&mut Self> {
    let typeid:TypeId = data.type_id();
    let index = self.inserting_into_index;

    if let Some(components) = self.components.get_mut(&typeid) {
      let component = components.get_mut(index).ok_or(EcsErrors::CreateComponentNeverCalled)?;
      *component = Some(Rc::new(RefCell::new(data)));

      let bitmask = self.bitmasks.get(&typeid).unwrap();
      self.map[index] |= *bitmask
    } else {
      dbg!();
      return Err(EcsErrors::CreateComponentNeverCalled.into());
    };
    Ok(self)
  }

  pub fn get_bitmask(&self, typeid:&TypeId) -> Option<u128> {
    return self.bitmasks.get(typeid).copied();
  }

  pub fn delete_component_by_entity_id<T:Any>(&mut self, index:usize) -> Result<()> {
    let typeid = TypeId::of::<T>();
    //what is happening here?
    let mask = if let Some(mask) = self.bitmasks.get(&typeid) {
      mask
    } else {
      return Err(EcsErrors::ComponentNotRegistered.into());
    };

    self.map[index] ^= *mask;

    Ok(())
  }

  //might wanna modify the add component to also be used for updating *or* make
  // an update query based on it
  pub fn add_component_by_entity_id(&mut self, data:impl Any, index:usize) -> Result<()> {
    let typeid = data.type_id();

    let mask = if let Some(mask) = self.bitmasks.get(&typeid) {
      mask
    } else {
      return Err(EcsErrors::ComponentNotRegistered.into());
    };

    self.map[index] |= *mask;

    let components = self.components.get_mut(&typeid).unwrap();
    components[index] = Some(Rc::new(RefCell::new(data)));

    Ok(())
  }

  pub fn delete_entity(&mut self, index:usize) -> Result<()> {
    if let Some(map) = self.map.get_mut(index) {
      *map = 0;
    } else {
      return Err(EcsErrors::EntityDoesNotExist.into());
    }

    Ok(())
  }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
  use std::any::TypeId;

  use super::*;

  #[test]
  fn register_an_entity() {
    let mut entities:Entities = Entities::default();
    let typeid = TypeId::of::<Health>();
    entities.register_component::<Health>();
    let health_components = entities.components.get(&typeid).unwrap();
    assert_eq!(health_components.len(), 0);
  }

  #[test]
  fn bitmask_updated_when_register_an_entity() {
    let mut entities:Entities = Entities::default();

    entities.register_component::<Health>();
    let typeid = TypeId::of::<Health>();
    let mask = entities.bitmasks.get(&typeid).unwrap();
    assert_eq!(*mask, 1);

    entities.register_component::<Speed>();
    let typeid = TypeId::of::<Speed>();
    let mask = entities.bitmasks.get(&typeid).unwrap();
    assert_eq!(*mask, 2);
  }

  #[test]
  fn create_an_entity() {
    let mut entities:Entities = Entities::default();
    entities.register_component::<Health>();
    entities.register_component::<Speed>();
    entities.create_entity();
    let health = entities.components.get(&TypeId::of::<Health>()).unwrap();
    let speed = entities.components.get(&TypeId::of::<Speed>()).unwrap();
    assert!(health.len() == speed.len() && health.len() == 1);
    assert!(health[0].is_none() && speed[0].is_none());
  }

  #[test]
  fn create_with_component() -> Result<()> {
    let mut entities:Entities = Entities::default();
    entities.register_component::<Health>();
    entities.register_component::<Speed>();
    entities
      .create_entity()
      .with_component(Health(100))?
      .with_component(Speed(15))?;

    let first_health = &entities.components.get(&TypeId::of::<Health>()).unwrap()[0];
    let wrapped_health = first_health.as_ref().unwrap();
    let borrowed_health = wrapped_health.borrow();
    let health = borrowed_health.downcast_ref::<Health>().unwrap();
    assert_eq!(health.0, 100);
    Ok(())
  }

  #[test]
  fn map_is_updated_when_creating_entities() -> Result<()> {
    let mut entities:Entities = Entities::default();

    entities.register_component::<Health>();
    entities.register_component::<Speed>();

    entities.create_entity().with_component(Health(100))?.with_component(Speed(15))?;

    let entity_map = entities.map[0];
    assert_eq!(entity_map, 3);

    entities.create_entity().with_component(Speed(15))?;

    let entity_map = entities.map[1];
    assert_eq!(entity_map, 2);

    Ok(())
  }

  #[test]
  fn delete_component_by_entity_id() -> Result<()> {
    let mut entities = Entities::default();

    entities.register_component::<Health>();
    entities.register_component::<Speed>();

    entities.create_entity().with_component(Health(100))?.with_component(Speed(50))?;

    entities.delete_component_by_entity_id::<Health>(0)?;

    assert_eq!(entities.map[0], 2);

    Ok(())
  }

  #[test]
  fn add_component_to_entity_by_id() -> Result<()> {
    let mut entities = Entities::default();

    entities.register_component::<Health>();
    entities.register_component::<Speed>();

    entities.create_entity().with_component(Health(100))?;

    //how are we finding the entity's id?
    entities.add_component_by_entity_id(Speed(50), 0)?;

    assert_eq!(entities.map[0], 3);

    // what is all of this code doing?
    let speed_typeid = TypeId::of::<Speed>();
    let wrapped_speeds = entities.components.get(&speed_typeid).unwrap();
    let wrapped_speed = wrapped_speeds[0].as_ref().unwrap();
    let borrowed_speed = wrapped_speed.borrow();
    let speed = borrowed_speed.downcast_ref::<Speed>().unwrap();

    assert_eq!(speed.0, 50);

    Ok(())
  }

  #[test]
  fn delete_entity_by_id() -> Result<()> {
    let mut entities = Entities::default();

    entities.register_component::<Health>();

    entities.create_entity().with_component(Health(100))?;

    entities.delete_entity(0)?;

    assert_eq!(entities.map[0], 0);

    Ok(())
  }

  #[test]
  fn created_entities_are_inserted_into_deleted_entities_columns() -> Result<()> {
    let mut entities = Entities::default();
    entities.register_component::<Health>();
    entities.register_component::<Speed>();

    entities.create_entity().with_component(Health(100))?;

    entities.create_entity().with_component(Health(50))?;

    entities.delete_entity(0)?;

    entities.create_entity().with_component(Health(25))?;

    assert_eq!(entities.map[0], 1);

    let typeid = TypeId::of::<Health>();
    let borrowed_health = entities.components.get(&typeid).unwrap()[0].as_ref().unwrap().borrow();

    let health = borrowed_health.downcast_ref::<Health>().unwrap();

    assert_eq!(health.0, 25);

    Ok(())
  }

  struct Health(pub u32);
  struct Speed(pub u32);
}
