use crate::{
  ecs::{component_lib::{Position, SkinnedMesh}, world_resources::RenderUniformLocations, World},
  math::{Transforms, Vec3},
  view::render_gl::Program
};
use eyre::Result;
use gl::Gl;
use glm::lerp;

use super::render_mesh::render_mesh;

pub fn skinned_meshes(world:&World, program:&Program, interpolation_factor:f64) -> Result<()> {
  let gl = world.immut_get_resource::<Gl>().unwrap();
  let transforms = world.immut_get_resource::<Transforms>().unwrap();
  let uniform_locations = world.immut_get_resource::<RenderUniformLocations>().unwrap();

  let mut query = world.query();

  let entities = query.with_component::<SkinnedMesh>()?.with_component::<Position>()?.run_entity();

  for entity in entities {
    let position = entity.immut_get_component::<Position>()?;
    //this is smoother but starts jerking around at high speeds
    let render_position:Vec3 = lerp(&position.tick_start, &position.tick_end, interpolation_factor as f32);
    let mesh = entity.immut_get_component::<SkinnedMesh>()?;
    program.set_uniform_matrix4fv(gl, uniform_locations.model, &transforms.get_model_transform(&render_position, 1.0));
    render_mesh(gl, &mesh.0)
  }

  Ok(())
}
