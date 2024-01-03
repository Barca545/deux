use crate::{
  ecs::{component_lib::{Position, SkinnedMesh}, world_resources::ShaderPrograms, World},
  math::{Vec3, calculate_model_transform, math::Mat4},
};
use eyre::Result;
use gl::Gl;
use glm::lerp;

use super::render_mesh::render_mesh;

pub fn skinned_meshes(world:&World, interpolation_factor:f64) -> Result<()> {
  let gl = world.immut_get_resource::<Gl>().unwrap();
  let program = world.immut_get_resource::<ShaderPrograms>().unwrap().normal;

  let mut query = world.query();

  let entities = query.with_component::<SkinnedMesh>()?.with_component::<Position>()?.run_entity();

  for entity in entities {
    let position = entity.immut_get_component::<Position>()?;
    //this is smoother but starts jerking around at high speeds
    let render_position:Vec3 = lerp(&position.tick_start, &position.tick_end, interpolation_factor as f32);
    
    //Calculate the model transform
    let mesh = entity.immut_get_component::<SkinnedMesh>()?;
    let model_transform:Mat4 = calculate_model_transform(&render_position, mesh.scale_factor);
    
    //Set the model transform's value
    program.set_model_matrix(gl, &model_transform);
    
    render_mesh(gl, &mesh.mesh);
  }
  
  Ok(())
}
