use crate::{
  ecs::{component_lib::{Position, StaticMesh}, world_resources::RenderUniformLocations, World},
  math::{Transforms, Vec3},
  view::render_gl::Program
};
use eyre::Result;
use gl::Gl;

use super::render_mesh::render_mesh;

pub fn static_geometry(world:&World, program:&Program) -> Result<()> {
  let gl = world.immut_get_resource::<Gl>().unwrap();
  let transforms = world.immut_get_resource::<Transforms>().unwrap();
  let uniform_locations = world.immut_get_resource::<RenderUniformLocations>().unwrap();

  let mut query = world.query();

  let entities = query.with_component::<StaticMesh>()?.with_component::<Position>()?.run_entity();

  for entity in entities {
    let position = entity.immut_get_component::<Position>()?;
    //this is smoother but starts jerking around at high speeds
    let render_position:Vec3 = position.tick_end;
    let mesh = entity.immut_get_component::<StaticMesh>()?;

    //bind the model transform
    program.set_uniform_matrix4fv(gl, uniform_locations.model, &transforms.get_model_transform(&render_position, 1.0));
    render_mesh(gl, &mesh.0);
  }
  Ok(())
}
