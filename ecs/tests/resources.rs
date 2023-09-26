
use std::path::PathBuf;
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

#[test]
fn remove_resource(){
  let mut world:World = init_world();
  world.remove_resource::<FpsResource>();
  let deleted_resource = world.immut_get_resource::<FpsResource>();
  assert!(deleted_resource.is_none())
}

#[test]
fn add_resource_by_type(){
  let mut world = init_world();

  world.add_resource().from_user_defined_data(FpsResource(60));
  let fps = world.immut_get_resource::<FpsResource>().unwrap();
  assert_eq!(fps.0,60);

  world.add_resource().folder_from_relative_exe_path("assets");
  let path:&PathBuf = world.immut_get_resource::<PathBuf>().unwrap();
  assert_eq!(path.to_str(), Some("C:\\Users\\Jamari\\Documents\\Hobbies\\Coding\\deux\\target\\debug\\deps\\assets"));
}

#[test]
fn load_resource_from_cstring(){
  let mut world = init_world();

  world.add_resource().folder_from_relative_exe_path("assets");
  let path:&PathBuf = world.immut_get_resource::<PathBuf>().unwrap();
  assert_eq!(path.to_str(), Some("C:\\Users\\Jamari\\Documents\\Hobbies\\Coding\\deux\\target\\debug\\deps\\assets"));
  
  // let cstring = world.load_resource_from_cstring("triangle.vert").unwrap();
}

fn init_world() -> World {
  let mut world: World = World::new();
  world.add_resource().from_user_defined_data(FpsResource(60));
  return world
}

#[derive(Debug)]
struct FpsResource(pub u32);

impl std::ops::Deref for FpsResource {
  type Target = u32;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}