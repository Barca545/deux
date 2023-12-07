use std::any::{Any, TypeId};
use eyre::Result;
use crate::{errors::EcsErrors, ecs::entities::{Component, Entities}};

use super::query_entity::QueryEntity;
// use super::{entities::{Entities, Component}, query_entity::QueryEntity};

pub type QueryIndices = Vec<usize>;
pub type QueryComponents = Vec<Vec<Component>>;

//Redo the Query: https://www.youtube.com/watch?v=PIyCUYcJefo&list=PLrmY5pVcnuE_SQSzGPWUJrf9Yo-YNeBYs&index=47

#[derive(Debug)]
pub struct Query<'a> {
  map: u32,
  entities: &'a Entities,
  typeids: Vec<TypeId>,
}

impl<'a> Query<'a> {
  pub fn new(entities:&'a Entities) -> Self{
    Self {
      map:0,
      entities,
      typeids:vec![]
    }
  }
  ///Tells the query the entities it pulls must contain the component passed in as an argument.
  pub fn with_component<T: Any>(&mut self) -> Result<&mut Self>{
    let typeid = TypeId::of::<T>();
    if let Some(bit_mask) = self.entities.get_bitmask(&typeid){
      self.map |= bit_mask; 
      self.typeids.push(typeid)
    }
    else {
      return Err(EcsErrors::ComponentNotRegistered.into());
    }
    Ok(self)
  }

  pub fn with_component_typeid(&mut self,typeid:TypeId) -> Result<&mut Self>{
    if let Some(bit_mask) = self.entities.get_bitmask(&typeid){
      self.map |= bit_mask; 
      self.typeids.push(typeid)
    }
    else {
      return Err(EcsErrors::ComponentNotRegistered.into());
    }
    Ok(self)
  }
  
  ///Returns entities from entities containing all of the components in a whose bitmask matches the query's bitmask
  #[deprecated(since="Forever",note="Query Entity method does it better")]
  pub fn run(&self) -> (QueryIndices,QueryComponents){
    //what exactly are these indexes?
    let indexes:Vec<usize> = self
      .entities
      .map
      .iter()
      .enumerate()
      .filter_map(|(index, entity_map)|{
        if entity_map & self.map == self.map {
          Some(index)
        }
        else {
          None
        }
    })
    .collect();
    
    let mut result = vec![];

    for typeid in &self.typeids {
      let entity_components = self.entities.components.get(typeid).unwrap();
      let mut components_to_keep = vec![];
      for index in &indexes {
        components_to_keep.push(entity_components[*index].as_ref().unwrap().clone())
      }
      result.push(components_to_keep);
    };
    return (indexes,result)
  }
  
  ///Returns a query entity containing all entities containing the queried components. 
  ///Exposes the `immut_get_component` and `mut_get_component` methods for returned entities.
  pub fn run_entity(&self) -> Vec<QueryEntity>{
    self
      .entities
      .map
      .iter()
      .enumerate()
      .filter_map(|(index, entity_map)|{
        if entity_map & self.map == self.map {
          Some(QueryEntity::new(index,self.entities))
        }
        else {
          None
        }
    })
    .collect()
  }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod test {
  // use super::entities::query_entity::QueryEntity;
  use std::cell::{Ref, RefMut};
  use super::*;

  #[test]
  fn query_mask_updating_with_component() -> Result<()>{
    let mut entities: Entities = Entities::default();

    entities.register_component::<u32>();
    entities.register_component::<f32>();

    let mut query: Query<'_> =  Query::new(&entities);
    
    query
      .with_component::<u32>()?
      .with_component::<f32>()?;

    assert_eq!(query.map, 3);
    assert_eq!(TypeId::of::<u32>(),query.typeids[0]); 
    assert_eq!(TypeId::of::<f32>(),query.typeids[1]); 
    Ok(())
  }
  

  #[test]
  fn run_query () -> Result<()> {
    let mut entities: Entities = Entities::default();
    entities.register_component::<u32>();
    entities.register_component::<f32>();

    entities
      .create_entity()
      .with_component(32_u32)?
      .with_component(40.0_f32)?;

    entities.create_entity()
      .with_component(5.0_f32)?;

    entities.create_entity()
      .with_component(87_u32)?;

    entities
      .create_entity()
      .with_component(15_u32)?
      .with_component(25.0_f32)?;

    let mut query: Query<'_> =  Query::new(&entities);
    
    query
      .with_component::<u32>()?
      .with_component::<f32>()?;
      
    let query_result =  query.run();
    
    let u32s = &query_result.1[0];
    let f32s = &query_result.1[1];
    let indexes = query_result.0;

    assert!(u32s.len()==f32s.len() && u32s.len()==indexes.len());
    assert_eq!(u32s.len(),2);
    
    let borrowed_first_u32 = u32s[0].borrow();
    let first_u32: &u32 = borrowed_first_u32.downcast_ref::<u32>().unwrap();
    assert_eq!(*first_u32,32);

    let borrowed_first_f32 = f32s[0].borrow();
    let first_f32: &f32 = borrowed_first_f32.downcast_ref::<f32>().unwrap();
    assert_eq!(*first_f32,40.0);

    let borrowed_second_u32 = u32s[1].borrow();
    let second_u32: &u32 = borrowed_second_u32.downcast_ref::<u32>().unwrap();
    assert_eq!(*second_u32, 15);

    let borrowed_second_f32 = f32s[1].borrow();
    let second_f32: &f32 = borrowed_second_f32.downcast_ref::<f32>().unwrap();
    assert_eq!(*second_f32, 25.0);

    //not sure why this is returning an array but I kinda understand what is happening 
    //the first element in the indexes array should be the map of the first type 
    assert_eq!(indexes[0],0);
    assert_eq!(indexes[1],3);

    Ok(())
  }

  #[test]
  fn query_for_entity_ref() -> Result<()>{
    let mut entities = Entities::default();
    entities.register_component::<u32>();
    entities.register_component::<f32>();

    entities.create_entity()
      .with_component(100_u32)?;

    entities.create_entity()
      .with_component(10.0_f32)?;

    let mut query = Query::new(&entities);
    
    let entities: Vec<QueryEntity> = query
    .with_component::<u32>()?
    .run_entity();

  assert_eq!(entities.len(),1);
  
  for entity in entities {
    assert_eq!(entity.id, 0);
    let health: Ref<u32> = entity.immut_get_component::<u32>()?;
    assert_eq!(*health, 100);
  }
  Ok(())
  }

  #[test]
  fn query_for_entity_mutable() -> Result<()>{
    let mut entities = Entities::default();
    entities.register_component::<u32>();
    entities.register_component::<f32>();

    entities.create_entity()
      .with_component(100_u32)?;

    entities.create_entity()
      .with_component(10.0_f32)?;

    let mut query = Query::new(&entities);
    
    let entities: Vec<QueryEntity> = query
    .with_component::<u32>()?
    .run_entity();

  assert_eq!(entities.len(),1);
  
  for entity in entities {
    assert_eq!(entity.id, 0);
    let mut health: RefMut<u32> = entity.mut_get_component::<u32>()?;
    assert_eq!(*health, 100);
    *health += 1;
  }

  let entities: Vec<QueryEntity> = query
    .with_component::<u32>()?
    .run_entity();
  
  for entity in entities {
    let health: Ref<u32> = entity.immut_get_component::<u32>()?;
    assert_eq!(*health, 101);
  }
  Ok(())
  }
}
