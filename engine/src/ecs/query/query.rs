use super::query_entity::QueryEntity;
use crate::{ecs::entities::Entities, errors::EcsErrors};
use eyre::Result;
use std::any::{Any, TypeId};

// Refactor:
// -Alternate query branch statement: (entity_map & self.map == self.map) && (entity_map & self.exlude_map == 0)

#[derive(Debug)]
pub struct Query<'a> {
  map: u128,
  exclude_map: u128,
  entities: &'a Entities,
}

impl<'a> Query<'a> {
  ///Create a new [`Query`].
  pub fn new(entities: &'a Entities) -> Self {
    Self {
      map: 0,
      exclude_map: 0,
      entities,
    }
  }

  ///Add a component the queried entities must hold.
  pub fn with_component<T: Any>(&mut self) -> Result<&mut Self> {
    let typeid = TypeId::of::<T>();
    if let Some(bit_mask) = self.entities.get_bitmask(&typeid) {
      self.map |= bit_mask;
    } else {
      return Err(EcsErrors::ComponentNotRegistered.into());
    }
    Ok(self)
  }

  ///Add a component the queried entities must not hold.
  pub fn without_component<T: Any>(&mut self) -> Result<&mut Self> {
    let typeid = TypeId::of::<T>();
    if let Some(bit_mask) = self.entities.get_bitmask(&typeid) {
      self.exclude_map |= bit_mask;
    } else {
      return Err(EcsErrors::ComponentNotRegistered.into());
    }
    Ok(self)
  }

  ///Consumes the [`Query`]. Returns a [`Vec`] of [`QueryEntity`] containing all entities who hold the queried components.
  pub fn run(&self) -> Vec<QueryEntity> {
    self
      .entities
      .map
      .iter()
      .enumerate()
      .filter_map(|(index, entity_map)| {
        if (entity_map & (self.map | self.exclude_map)) == self.map {
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
  use super::*;
  use std::cell::{Ref, RefMut};
  #[test]
  fn query_mask_updating_with_component() -> Result<()> {
    let mut entities: Entities = Entities::default();

    entities.register_component::<u32>();
    entities.register_component::<f32>();

    let mut query: Query<'_> = Query::new(&entities);

    query.with_component::<u32>()?.with_component::<f32>()?.without_component::<usize>()?;

    assert_eq!(query.map, 3);
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
    Ok(())
  }

  #[test]
  fn query_for_entity_without_component() -> Result<()> {
    let mut entities = Entities::default();
    entities.register_component::<Health>();
    entities.register_component::<Damage>();
    entities.register_component::<usize>();
    entities.register_component::<f32>();

    entities.create_entity().with_component(Damage(100))?;
    entities.create_entity().with_component(Damage(100))?.with_component(Health(100))?;
    entities.create_entity().with_component(Health(30))?.with_component(5_usize)?;

    let mut query = Query::new(&entities);
    let entities = query.with_component::<Health>()?.without_component::<Damage>()?.run();

    assert_eq!(entities.len(), 1);

    let entity = &entities[0];
    let health = entity.get_component::<Health>()?;
    assert_eq!(health.0, 30);

    Ok(())
  }
  struct Health(pub i32);
  struct Damage(pub u32);
}
