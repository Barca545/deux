use crate::{
  component_lib::{Position, PreviousPosition, SkinnedMesh}, ecs::{world_resources::ShaderPrograms, World}, math::{calculate_model_transform, math::Mat4, Vec3}
};
use gl::Gl;
use glm::lerp;
use super::render_mesh::render_mesh;

// Refactor
// -Figure out why the lerping still jitters at high speeds
// -Evaluate if the linear interpolation is needed at all since setting the render position equal to position seems to work fine

pub fn skinned_meshes(world:&World, interpolation_factor:f64) {
  let gl = world.immut_get_resource::<Gl>().unwrap();
  let program = world.immut_get_resource::<ShaderPrograms>().unwrap().normal;

  let mut query = world.query();
  let entities = query.with_component::<SkinnedMesh>().unwrap().with_component::<Position>().unwrap().with_component::<PreviousPosition>().unwrap().run();

  for entity in entities {
    //Get the render position by lerping between the position at the end of the previous game logic tick and the position at the end of the current game logic tick
    let position = entity.immut_get_component::<Position>().unwrap();
    let previous_position = entity.immut_get_component::<PreviousPosition>().unwrap();
    let render_position:Vec3 = calculate_render_position(*previous_position, *position, interpolation_factor);
    

    //Calculate the model transform
    let mesh = entity.immut_get_component::<SkinnedMesh>().unwrap();
    let model_transform:Mat4 = calculate_model_transform(&render_position, mesh.scale_factor);
    
    //Set the model transform's value
    program.set_model_matrix(gl, &model_transform);
    
    render_mesh(gl, &mesh.mesh);
  }
}


fn calculate_render_position(previous_position:PreviousPosition, position:Position, interpolation_factor:f64)->Vec3{
  let render_position:Vec3;
  if 
  previous_position.0.x == position.0.x
  &&
  previous_position.0.y == position.0.y
  &&
  previous_position.0.z == position.0.z 
  {
    render_position = lerp(&previous_position.0, &position.0, interpolation_factor as f32);
  }
  else {
    render_position = position.0;
  }
  render_position
}