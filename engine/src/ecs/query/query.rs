use super::query_entity::QueryEntity;
use crate::{ecs::entities::Entities, errors::EcsErrors};
use eyre::Result;
use std::any::{Any, TypeId};

#[derive(Debug)]
pub struct Query<'a> {
  map: u128,
  entities: &'a Entities,
  typeids: Vec<TypeId>,
}

impl<'a> Query<'a> {
  pub fn new(entities: &'a Entities) -> Self {
    Self {
      map: 0,
      entities,
      typeids: vec![],
    }
  }
  ///Tells the query the entities it pulls must contain the type passed in
  /// as an argument.
  pub fn with_component<T: Any>(&mut self) -> Result<&mut Self> {
    let typeid = TypeId::of::<T>();
    if let Some(bit_mask) = self.entities.get_bitmask(&typeid) {
      self.map |= bit_mask;
      self.typeids.push(typeid)
    } else {
      return Err(EcsErrors::ComponentNotRegistered.into());
    }
    Ok(self)
  }

  ///Returns a query entity containing all entities containing the queried
  /// components. Exposes the `get_component` and `mut_get_component`
  /// methods for returned entities.
  pub fn run(&self) -> Vec<QueryEntity> {
    self
      .entities
      .map
      .iter()
      .enumerate()
      .filter_map(|(index, entity_map)| {
        if entity_map & self.map == self.map {
          Some(QueryEntity::new(index, self.entities))
        } else {
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
  use super::*;
  use std::cell::{Ref, RefMut};

  #[test]
  fn query_mask_updating_with_component() -> Result<()> {
    let mut entities: Entities = Entities::default();

    entities.register_component::<u32>();
    entities.register_component::<f32>();

    let mut query: Query<'_> = Query::new(&entities);

    query.with_component::<u32>()?.with_component::<f32>()?;

    assert_eq!(query.map, 3);
    assert_eq!(TypeId::of::<u32>(), query.typeids[0]);
    assert_eq!(TypeId::of::<f32>(), query.typeids[1]);
    Ok(())
  }

  #[test]
  fn query_for_entity_ref() -> Result<()> {
    let mut entities = Entities::default();
    entities.register_component::<u32>();
    entities.register_component::<f32>();

    entities.create_entity().with_component(100_u32)?;

    entities.create_entity().with_component(10.0_f32)?;

    let mut query = Query::new(&entities);

    let entities: Vec<QueryEntity> = query.with_component::<u32>()?.run();

    assert_eq!(entities.len(), 1);

    for entity in entities {
      assert_eq!(entity.id, 0);
      let health: Ref<u32> = entity.get_component::<u32>()?;
      assert_eq!(*health, 100);
    }
    Ok(())
  }

  #[test]
  fn query_for_entity_mutable() -> Result<()> {
    let mut entities = Entities::default();
    entities.register_component::<Health>();
    entities.register_component::<f32>();

    entities.create_entity().with_component(Health(100))?;

    entities.create_entity().with_component(10.0_f32)?;

    let mut query = Query::new(&entities);

    let entities: Vec<QueryEntity> = query.with_component::<Health>()?.run();

    assert_eq!(entities.len(), 1);

    for entity in entities {
      assert_eq!(entity.id, 0);
      let mut health: RefMut<Health> = entity.get_component_mut::<Health>()?;
      assert_eq!(health.0, 100);
      health.0 += 1;
    }

    let entities: Vec<QueryEntity> = query.with_component::<Health>()?.run();

    for entity in entities {
      let health: Ref<Health> = entity.get_component::<Health>()?;
      assert_eq!(health.0, 101);
    }
    Ok(())
  }

  #[test]
  fn query_for_entity_after_component_delete() -> Result<()> {
    let mut entities = Entities::default();
    entities.register_component::<Health>();
    entities.register_component::<Damage>();

    entities.create_entity().with_component(Health(100))?;
    entities.add_component_by_entity_id(0, Damage(100))?;
    entities.delete_component_by_entity_id::<Damage>(0)?;

    let mut query = Query::new(&entities);

    let entities = query.with_component::<Health>()?.with_component::<Damage>()?.run();
    assert_eq!(entities.len(), 0);
    // let entity = &entities[0];
    //len should be zero
    //

    Ok(())
  }
  struct Health(pub i32);
  struct Damage(pub u32);
}
