use eyre::Result;
use gl::Gl;

use crate::{ecs::{World, component_lib::{Position, StaticMesh}}, filesystem::load_object, math::Vec3};

pub fn spawn_enviroment(world:&mut World, name:&str) -> Result<()> {
  let gl: &Gl = world.immut_get_resource::<Gl>().unwrap();

  let position_vec:Vec3 = Vec3::new(0.0, -0.5, 0.0);
  //maybe make a static component that holds the position
  let position = Position::new(position_vec, position_vec);
  let (vertices, indices) = load_object(name)?;
  let mesh = StaticMesh::new(&gl,vertices, indices,name);
  
  world
    .create_entity()
    .with_component(mesh)?
    .with_component(position)?;
  
  Ok(())
}