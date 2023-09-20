use std::any::{Any, TypeId};
use eyre::Result;
use crate::custom_errors::CustomErrors;
use super::entities::{Entities, Component};

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
  pub fn with_component<T: Any>(& mut self) -> Result<&mut Self>{
    let typeid = TypeId::of::<T>();
    if let Some(bit_mask) = self.entities.get_bitmask(&typeid){
      self.map |= bit_mask; 
      self.typeids.push(typeid)
    }
    else {
      return Err(CustomErrors::ComponentNotRegistered.into());
    }
    Ok(self)
  }
  
  ///Returns entities from entities containing all of the components in map i.e. all of the entities whose bitmask matches the query's bitmask
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
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod test {
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
}
