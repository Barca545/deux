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

    //I think I can abstract this into another function and share it between the skinned and static meshes
    let mesh = entity.immut_get_component::<SkinnedMesh>()?;
    // let texture = &mesh.0.texture;
    // let vao = &mesh.0.vao;

    //do I bind texture before or after vao
    // texture.bind(gl);
    // vao.bind();

    //bind the model transform
    //experiment with binding this outside of the draw function
    program.set_uniform_matrix4fv(gl, uniform_locations.model, &transforms.get_model_transform(&render_position, 1.0));

    // unsafe {
    //   gl.DrawArrays(TRIANGLES, 0, 36);
    // }
    // vao.unbind();
    render_mesh(gl, &mesh.0)
  }

  Ok(())
}
