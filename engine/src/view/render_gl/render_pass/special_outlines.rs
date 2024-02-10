use crate::{
  component_lib::{Position, PreviousPosition}, ecs::{
    component_lib::SkinnedMesh,
    world_resources::{Selected::{self, HOVERED}, ShaderPrograms},
    World
  }, math::{calculate_model_transform, Vec3}
};
use eyre::Result;
use gl::{Gl, TRIANGLES};
use glm::lerp;

pub fn special_outlines(world:&World, interpolation_factor:f64) -> Result<()> {
  let gl = world.immut_get_resource::<Gl>().unwrap();
  let selection = world.immut_get_resource::<Selected>().unwrap();

  //probably need to make selected a vec and so still need a loop
  match selection {
    HOVERED(id) => {
      let mesh = world.immut_get_component_by_entity_id::<SkinnedMesh>(*id)?;
      let program = world.immut_get_resource::<ShaderPrograms>().unwrap().normal;
      
      //Get the render position by lerping between the position at the end of the previous game logic tick and the position at the end of the current game logic tick
      let position = world.immut_get_component_by_entity_id::<Position>(*id)?;
      let previous_position = world.immut_get_component_by_entity_id::<PreviousPosition>(*id)?;
      let render_position:Vec3 = lerp(&previous_position.0, &position.0, interpolation_factor as f32);

      let texture = &mesh.mesh.texture;
      let vao = &mesh.mesh.vao;

      texture.bind(gl);
      vao.bind(gl);

      //Bind the model transform
      let model_transform = calculate_model_transform(&render_position, 1.1);
      
      //Set the model transform's value
      program.set_model_matrix(gl, &model_transform);

      unsafe {
        gl.DrawArrays(TRIANGLES, 0, 36);
      }
      vao.unbind(gl);
    }
    _ => {}
  }
  Ok(())
}
