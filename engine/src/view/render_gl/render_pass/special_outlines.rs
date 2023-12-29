use crate::{
  ecs::{
    component_lib::{Position, SkinnedMesh},
    world_resources::{RenderUniformLocations, Selected, Selected::HOVERED},
    World
  },
  math::{Transforms, Vec3},
  view::render_gl::Program
};
use eyre::Result;
use gl::{Gl, TRIANGLES};
use glm::lerp;

pub fn special_outlines(world:&World, program:&Program, interpolation_factor:f64) -> Result<()> {
  let gl = world.immut_get_resource::<Gl>().unwrap();
  let transforms = world.immut_get_resource::<Transforms>().unwrap();
  let uniform_locations = world.immut_get_resource::<RenderUniformLocations>().unwrap();

  let selection = world.immut_get_resource::<Selected>().unwrap();

  //probably need to make selected a vec and so still need a loop
  match selection {
    HOVERED(id) => {
      let mesh = world.immut_get_component_by_entity_id::<SkinnedMesh>(*id)?;
      let position = world.immut_get_component_by_entity_id::<Position>(*id)?;

      let render_position:Vec3 = lerp(&position.tick_start, &position.tick_end, interpolation_factor as f32);

      let texture = &mesh.0.texture;
      let vao = &mesh.0.vao;

      texture.bind(gl);
      vao.bind(gl);

      //bind the model transform
      program.set_uniform_matrix4fv(gl, uniform_locations.model, &transforms.get_model_transform(&render_position, 1.1));

      unsafe {
        gl.DrawArrays(TRIANGLES, 0, 36);
      }
      vao.unbind(gl);
    }
    _ => {}
  }
  Ok(())
}
