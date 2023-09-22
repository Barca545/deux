use ecs::World;

#[test]
fn create_and_get_resource_immutably(){
  let world: World = init_world();

  let fps = world.immut_get_resource::<FpsResource>().unwrap();
  
  assert_eq!(fps.0,60)
}

#[test]
fn get_resources_mutably(){
  let mut world:World = init_world();
  //what does putting this in a block like this do?
  {
    let fps:&mut FpsResource = world.mut_get_resource::<FpsResource>().unwrap();
    fps.0 += 1
  }

  let fps:&FpsResource = world.immut_get_resource::<FpsResource>().unwrap();
  assert_eq!(fps.0,61)
}

fn init_world() -> World {
  let mut world: World = World::new();
  world.add_resource(FpsResource(60));
  dbg!(&world);
  return world
}

#[test]
fn remove_resource(){
  let mut world:World = init_world();
  world.remove_resource::<FpsResource>();
  let deleted_resource = world.immut_get_resource::<FpsResource>();
  assert!(deleted_resource.is_none())
}

#[derive(Debug)]
struct FpsResource(pub u32);

impl std::ops::Deref for FpsResource {
  type Target = u32;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}