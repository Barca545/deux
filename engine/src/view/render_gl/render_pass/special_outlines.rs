use crate::{
  ecs::{
    component_lib::{Position, SkinnedMesh},
    world_resources::{RenderUniformLocations, Selected, Selected::HOVERED},
    World
  },
  math::{Vec3, calculate_model_transform},
  view::render_gl::Program
};
use eyre::Result;
use gl::{Gl, TRIANGLES};
use glm::lerp;

pub fn special_outlines(world:&World, program:&Program, interpolation_factor:f64) -> Result<()> {
  let gl = world.immut_get_resource::<Gl>().unwrap();
  let uniform_locations = world.immut_get_resource::<RenderUniformLocations>().unwrap();

  let selection = world.immut_get_resource::<Selected>().unwrap();

  //probably need to make selected a vec and so still need a loop
  match selection {
    HOVERED(id) => {
      let mesh = world.immut_get_component_by_entity_id::<SkinnedMesh>(*id)?;
      let position = world.immut_get_component_by_entity_id::<Position>(*id)?;

      let render_position:Vec3 = lerp(&position.tick_start, &position.tick_end, interpolation_factor as f32);

      let texture = &mesh.mesh.texture;
      let vao = &mesh.mesh.vao;

      texture.bind(gl);
      vao.bind(gl);

      //bind the model transform
      let model_transform = calculate_model_transform(&render_position, 1.1);
      program.set_uniform_matrix4fv(gl, uniform_locations.model, &model_transform);

      unsafe {
        gl.DrawArrays(TRIANGLES, 0, 36);
      }
      vao.unbind(gl);
    }
    _ => {}
  }
  Ok(())
}
