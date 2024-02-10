use crate::{
  component_lib::{Position, PreviousPosition, SkinnedMesh}, ecs::{world_resources::ShaderPrograms, World}, math::{calculate_model_transform, math::Mat4, Vec3}
};
use eyre::Result;
use gl::Gl;
use glm::lerp;
use super::render_mesh::render_mesh;

// Refactor
// -Figure out why the lerping still jitters at high speeds

pub fn skinned_meshes(world:&World, interpolation_factor:f64) -> Result<()> {
  let gl = world.immut_get_resource::<Gl>().unwrap();
  let program = world.immut_get_resource::<ShaderPrograms>().unwrap().normal;

  let mut query = world.query();
  let entities = query.with_component::<SkinnedMesh>()?.with_component::<Position>()?.with_component::<PreviousPosition>()?.run_entity();

  for entity in entities {
    //Get the render position by lerping between the position at the end of the previous game logic tick and the position at the end of the current game logic tick
    let position = entity.immut_get_component::<Position>()?;
    let previous_position = entity.immut_get_component::<PreviousPosition>()?;
    let render_position:Vec3 = lerp(&previous_position.0, &position.0, interpolation_factor as f32);

    //Calculate the model transform
    let mesh = entity.immut_get_component::<SkinnedMesh>()?;
    let model_transform:Mat4 = calculate_model_transform(&render_position, mesh.scale_factor);
    
    //Set the model transform's value
    program.set_model_matrix(gl, &model_transform);
    
    render_mesh(gl, &mesh.mesh);
  }
  
  Ok(())
}
