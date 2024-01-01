use crate::{
  ecs::{
    component_lib::Position,
    world_resources::{DbgShaderProgram, RenderUniformLocations},
    World
  },
  math::{Vec3, calculate_model_transform},
  view::AABB3DDebugMesh
};
use eyre::Result;
use gl::{Gl, FILL, FRONT, LINE, LINES};
use glm::lerp;

pub fn debug(world:&World, interpolation_factor:f64) -> Result<()> {
  let gl = world.immut_get_resource::<Gl>().unwrap();
  let uniform_locations = world.immut_get_resource::<RenderUniformLocations>().unwrap();
  let dbg_program = world.immut_get_resource::<DbgShaderProgram>().unwrap();

  let mut query = world.query();

  let entities = query.with_component::<AABB3DDebugMesh>()?.with_component::<Position>()?.run_entity();

  for entity in entities {
    let position = entity.immut_get_component::<Position>()?;
    //this is smoother but starts jerking around at high speeds
    let render_position:Vec3 = lerp(&position.tick_start, &position.tick_end, interpolation_factor as f32);

    let mesh = entity.immut_get_component::<AABB3DDebugMesh>()?;
    let vao = &mesh.vao;

    vao.bind(gl);

    //bind the model transform
    let model_transform = calculate_model_transform(&render_position, 1.1);
    dbg_program
      .program
      .set_uniform_matrix4fv(gl, uniform_locations.model, &model_transform);

    unsafe {
      gl.PolygonMode(FRONT, LINE);
      gl.DrawArrays(LINES, 0, 36);
      gl.PolygonMode(FRONT, FILL);
    }
    vao.unbind(gl);
  }
  Ok(())
}
