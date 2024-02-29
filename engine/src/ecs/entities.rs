use super::bundle::{Bundle, TypeInfo};
use crate::errors::EcsErrors;
use eyre::Result;
use std::{
  any::{type_name, Any, TypeId},
  cell::RefCell,
  collections::HashMap,
  rc::Rc,
};

// Refactor:
// -Make components private
// -Instead of calling not registered could add components just register the
// -Use https://lib.rs/crates/list-any instead of Rc<RefCell<dyn Any>>;
// -Look into sparse arrays? https://discord.com/channels/676678179678715904/677286494033018924/1207030703888531457
// -Figure out if I can wrap the bitmasks in a RefCell so I can mutate them from a reference this would allow mutation in QueryEntity so I could have add/delete functionality
// -See about getting rid of the Rc so stuff can be contiguous in memory
// -look into a component trait like specs has that does sized + any https://docs.rs/specs/latest/src/specs/world/comp.rs.html#63-71

//From YT: https://www.youtube.com/watch?v=_9fAYLWSBVE&list=PLrmY5pVcnuE_SQSzGPWUJrf9Yo-YNeBYs&index=42
//I think a better approach would be to have a vec of free spots/indexes and
// that we keep adding to whenever we delete an entity, then when we add an
// entity, we look at the last available index in the free spots vec
// and we insert the components at that index, then we simply just .pop() on the
// vec of free indexes, this should be faster for large amounts of entities This
// will be used a lot so making it as fast as possible is good

pub type Component = Rc<RefCell<Box<dyn Any>>>;
pub type Components = HashMap<TypeId, Vec<Option<Component>>>;

#[derive(Debug, Default)]
pub struct Entities {
  pub components: Components,
  bitmasks: HashMap<TypeId, u128>,
  //contains the bitmaps for the registered components
  pub map: Vec<u128>,
  inserting_into_index: usize,
}

impl Entities {
  pub fn register_component<T: Any + 'static>(&mut self) -> &mut Self {
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

  pub fn reserve_entity(&mut self) -> usize {
    if let Some((index, _)) = self.map.iter().enumerate().find(|(_index, mask)| **mask == 0) {
      self.inserting_into_index = index;
    } else {
      self.components.iter_mut().for_each(|(_key, components)| components.push(None));
      self.map.push(0);
      self.inserting_into_index = self.map.len() - 1;
    }
    self.inserting_into_index
  }

  ///Used with `create_entity` to assign components and their initial values to
  /// the entity being created. Updates the entity's bitmap to indicate which components they contain.
  /// `with_component` will continue to update the same entity until a new entity is spawned.
  pub fn with_component<T: Any>(&mut self, data: T) -> Result<&mut Self> {
    let typeid: TypeId = data.type_id();
    let index = self.inserting_into_index;

    if let Some(components) = self.components.get_mut(&typeid) {
      let component = components.get_mut(index).ok_or(EcsErrors::CreateComponentNeverCalled {
        component: type_name::<T>().to_string(),
      })?;
      *component = Some(Rc::new(RefCell::new(Box::new(data))));

      let bitmask = self.bitmasks.get(&typeid).unwrap();
      self.map[index] |= *bitmask
    } else {
      #[cfg(debug_assertions)]
      return Err(
        EcsErrors::CreateComponentNeverCalled {
          component: type_name::<T>().to_string(),
        }
        .into(),
      );
    };
    Ok(self)
  }

  pub fn with_components(&mut self, bundle: impl Bundle) -> Result<()> {
    bundle.safe_put(|ty, data| {
      let typeid: TypeId = ty.id();
      let index = self.inserting_into_index;

      if let Some(components) = self.components.get_mut(&typeid) {
        let component = components
          .get_mut(index)
          .ok_or(EcsErrors::CreateComponentNeverCalled {
            component: ty.type_name().to_string(),
          })
          .unwrap();
        *component = Some(Rc::new(RefCell::new(data)));

        let bitmask = self.bitmasks.get(&typeid).unwrap();
        self.map[index] |= *bitmask
      }
    });
    Ok(())
  }

  pub fn get_bitmask(&self, typeid: &TypeId) -> Option<u128> {
    return self.bitmasks.get(typeid).copied();
  }

  pub fn delete_component_by_entity_id<T: Any>(&mut self, index: usize) -> Result<()> {
    let typeid = TypeId::of::<T>();
    if let Some(mask) = self.bitmasks.get(&typeid) {
      self.map[index] &= !*mask;
    }
    Ok(())
  }

  pub fn delete_component_by_type_info(&mut self, index: usize, ty: TypeInfo) -> Result<()> {
    if let Some(mask) = self.bitmasks.get(&ty.id()) {
      self.map[index] &= !*mask;
    }
    Ok(())
  }

  pub fn add_component_by_entity_id(&mut self, index: usize, data: impl Any) -> Result<()> {
    let typeid = data.type_id();

    if let Some(mask) = self.bitmasks.get(&typeid) {
      self.map[index] |= *mask;
    } else {
      return Err(EcsErrors::ComponentNotRegistered.into());
    };

    let components = self.components.get_mut(&typeid).unwrap();
    components[index] = Some(Rc::new(RefCell::new(Box::new(data))));

    Ok(())
  }

  pub fn add_components(&mut self, index: usize, components: impl Bundle) {
    // let test = self.components.get_mut(&typeinfo.id()).unwrap();
    components.safe_put(|typeinfo, data| {
      if let Some(mask) = self.bitmasks.get(&typeinfo.id()) {
        self.map[index] |= *mask;
      } else {
        dbg!(format!("Component {:?} Not Registered", typeinfo.id()));
      }
      let components = self.components.get_mut(&typeinfo.id()).unwrap();
      components[index] = Some(Rc::new(RefCell::new(data)));
    });
  }

  pub fn delete_entity(&mut self, index: usize) -> Result<()> {
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
  use super::*;
  use std::any::TypeId;

  #[test]
  fn register_an_entity() {
    let mut entities: Entities = Entities::default();
    let typeid = TypeId::of::<Health>();
    entities.register_component::<Health>();
    let health_components = entities.components.get(&typeid).unwrap();
    assert_eq!(health_components.len(), 0);
  }

  #[test]
  fn bitmask_updated_when_register_an_entity() {
    let mut entities: Entities = Entities::default();

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
    let mut entities: Entities = Entities::default();
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
    let mut entities: Entities = Entities::default();
    entities.register_component::<Health>();
    entities.register_component::<Speed>();
    entities.create_entity().with_component(Health(100))?.with_component(Speed(15))?;

    let first_health = &entities.components.get(&TypeId::of::<Health>()).unwrap()[0];
    let wrapped_health = first_health.as_ref().unwrap();
    let borrowed_health = wrapped_health.borrow();
    let health = borrowed_health.downcast_ref::<Health>().unwrap();
    assert_eq!(health.0, 100);
    Ok(())
  }

  #[test]
  fn map_is_updated_when_creating_entities() -> Result<()> {
    let mut entities: Entities = Entities::default();

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
    entities.register_component::<Damage>();

    entities
      .create_entity()
      .with_component(Health(100))?
      .with_component(Speed(50))?
      .with_component(Damage(50))?;

    assert_eq!(entities.map[0], 7);

    entities.delete_component_by_entity_id::<Health>(0)?;

    assert_eq!(entities.map[0], 6);

    Ok(())
  }

  #[test]
  fn add_component_to_entity_by_id() -> Result<()> {
    let mut entities = Entities::default();

    entities.register_component::<Health>();
    entities.register_component::<Speed>();

    entities.create_entity().with_component(Health(100))?;

    //how are we finding the entity's id?
    entities.add_component_by_entity_id(0, Speed(50))?;

    assert_eq!(entities.map[0], 3);

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
  struct Damage(pub u32);
}
