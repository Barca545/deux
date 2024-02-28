use eyre::Result;
use gl::Gl;
use crate::{component_lib::{Position, StaticMesh}, ecs::World, filesystem::load_object, math::Vec3};

pub fn spawn_enviroment(world:&mut World, name:&str) -> Result<()> {
  

  let position_vec:Vec3 = Vec3::new(0.0, -0.5, 0.0);
  let position = Position(position_vec);
  let (vertices, indices) = load_object(name)?;
  let mesh;
  {
    let gl = world.get_resource::<Gl>().unwrap();
    mesh = StaticMesh::new(&gl,vertices, indices,name);
  }
  world
    .create_entity()
    .with_component(mesh)?
    .with_component(position)?;

  Ok(())
}