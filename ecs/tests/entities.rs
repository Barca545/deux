use std::{any::Any, cell::RefCell, rc::Rc};

use ::eyre::Result;
use ecs::World;

#[test]
fn create_entity() -> Result<()> {
  let mut world:World = World::new();
  world.register_component::<Location>();
  world.register_component::<Size>();
  //what is this syntax
  world.create_entity().with_component(Location(42.0, 44.3))?.with_component(Size(32.0))?;
  Ok(())
}

#[test]
#[allow(clippy::float_cmp)]
fn query_for_entites() -> Result<()> {
  let mut world:World = World::new();
  world.register_component::<Location>();
  world.register_component::<Size>();

  world.create_entity().with_component(Location(42.0, 42.3))?.with_component(Size(32.0))?;

  world.create_entity().with_component(Location(55.0, 55.0))?;

  world.create_entity().with_component(Size(88.0))?;

  world.create_entity().with_component(Location(77.0, 77.3))?.with_component(Size(99.0))?;

  let query = world.query().with_component::<Location>()?.with_component::<Size>()?.run();

  let locations:&Vec<Rc<RefCell<dyn Any>>> = &query.1[0];
  let sizes:&Vec<Rc<RefCell<dyn Any>>> = &query.1[1];

  assert_eq!(locations.len(), sizes.len());
  assert_eq!(locations.len(), 2);
  assert_eq!(sizes.len(), 2);
  assert_eq!(locations.len(), 2);

  let borrowed_first_location = locations[0].borrow();
  let first_location = borrowed_first_location.downcast_ref::<Location>().unwrap();
  assert_eq!(first_location.0, 42.0);

  let borrowed_first_size = sizes[0].borrow();
  let first_size = borrowed_first_size.downcast_ref::<Size>().unwrap();
  assert_eq!(first_size.0, 32.0);

  let borrowed_second_location = locations[1].borrow();
  let second_location = borrowed_second_location.downcast_ref::<Location>().unwrap();
  assert_eq!(second_location.0, 77.0);

  let mut borrowed_second_size = sizes[1].borrow_mut();
  let second_size = borrowed_second_size.downcast_mut::<Size>().unwrap();
  second_size.0 += 1.0;
  assert_eq!(second_size.0, 100.0);

  Ok(())
}

#[test]
fn delete_component_from_entitiy() -> Result<()> {
  let mut world = World::new();
  world.register_component::<Size>();
  world.register_component::<Location>();

  world.create_entity().with_component(Location(10.0, 11.00))?.with_component(Size(10.0))?;

  world.create_entity().with_component(Location(20.0, 21.00))?.with_component(Size(20.0))?;

  world.delete_component_by_entity_id::<Size>(0)?;

  let query = world.query().with_component::<Size>()?.with_component::<Location>()?.run();

  assert_eq!(query.0.len(), 1);
  assert_eq!(query.0[0], 1);

  Ok(())
}

#[test]
fn add_component_to_entity() -> Result<()> {
  let mut world = World::new();

  world.register_component::<Location>();
  world.register_component::<Size>();

  world.create_entity().with_component(Location(10.0, 15.0))?;

  world.add_component_to_entity_by_id(Size(20.0), 0)?;

  let query = world.query().with_component::<Location>()?.with_component::<Size>()?.run();

  assert_eq!(query.0.len(), 1);

  Ok(())
}

#[test]
fn deleting_an_entity_by_id() -> Result<()> {
  let mut world = World::new();

  world.register_component::<Location>();
  world.register_component::<Size>();

  world.create_entity().with_component(Location(10.0, 15.0))?;

  world.create_entity().with_component(Location(20.0, 25.0))?;

  world.delete_entity_by_id(0)?;

  let query = world.query().with_component::<Location>()?.run();

  assert_eq!(query.0.len(), 1);

  let borrowed_location = query.1[0][0].borrow();
  let location = borrowed_location.downcast_ref::<Location>().unwrap();

  assert_eq!(location.0, 20.0);

  world.create_entity().with_component(Location(30.0, 35.0))?;

  let query = world.query().with_component::<Location>()?.run();

  let borrowed_location = query.1[0][0].borrow();
  let location = borrowed_location.downcast_ref::<Location>().unwrap();

  assert_eq!(location.0, 30.0);

  Ok(())
}

//test components
struct Location(pub f32, pub f32);
struct Size(pub f32);
