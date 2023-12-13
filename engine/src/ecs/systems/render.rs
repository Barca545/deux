use crate::{
  ecs::{
    component_lib::{
      ModelUniformLocation, Position, ProjectionUniformLocation, ViewUniformLocation,
    },
    World,
  },
  math::Transforms,
  view::{
    render::Mesh,
    render_gl::{buffer::VertexArray, Program, Texture},
  },
};
use eyre::Result;
use gl::{Gl, COLOR_BUFFER_BIT, DEPTH_BUFFER_BIT, TRIANGLES};

// pub fn render(world:&World,vao:&VertexArray) -> Result<()> {
//   let gl = world.immut_get_resource::<Gl>().unwrap();
//   let transforms = world.immut_get_resource::<Transforms>().unwrap();
//   let shader_program = world.immut_get_resource::<Program>().unwrap();
//   let model_uniform_loc = world.immut_get_resource::<ModelUniformLocation>().unwrap();
//   let view_uniform_loc = world.immut_get_resource::<ViewUniformLocation>().unwrap();
//   let projection_uniform_loc = world.immut_get_resource::<ProjectionUniformLocation>().unwrap();

//   let mut query = world.query();

//   let entities = query
//     .with_component::<Mesh>()?
//     .with_component::<Position>()?
//     .run_entity();

//   unsafe{gl.Clear(COLOR_BUFFER_BIT|DEPTH_BUFFER_BIT)};

//   for entity in entities {
//     let position = entity.immut_get_component::<Position>()?;

//     shader_program.set_uniform_matrix4fv(
//       0,
//       &transforms.get_model_transform(&position.0),
//     );

//     shader_program.set_uniform_matrix4fv(
//       3,
//       &transforms.get_view_transform()
//     );

//     shader_program.set_uniform_matrix4fv(
//       2,
//       transforms.get_projection_transform().as_matrix(),
//     );

//     //bind the texture
//     let texture = Texture::new(gl,"ground.jpg")?;
//     texture.bind(gl);
//     vao.bind();

//     unsafe {gl.DrawArrays(TRIANGLES, 0, 36)};

//     //glfw swap buffers
// }

//   Ok(())
// }

pub fn render(world: &World) -> Result<()> {
  let gl = world.immut_get_resource::<Gl>().unwrap();
  let transforms = world.immut_get_resource::<Transforms>().unwrap();
  let program = world.immut_get_resource::<Program>().unwrap();
  let model_uniform_loc = world.immut_get_resource::<ModelUniformLocation>().unwrap();
  let view_uniform_loc = world.immut_get_resource::<ViewUniformLocation>().unwrap();
  let projection_uniform_loc = world
    .immut_get_resource::<ProjectionUniformLocation>()
    .unwrap();

  let mut query = world.query();

  let entities = query
    .with_component::<Mesh>()?
    .with_component::<Position>()?
    .run_entity();

  unsafe { gl.Clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT) };

  for entity in entities {
    let position = entity.immut_get_component::<Position>()?;
    let mesh = entity.immut_get_component::<Mesh>()?;
    let texture = &mesh.texture;
    let vao = &mesh.vao;

    //should all the shader stuff be wrapped into a method on the struct?
    program.use_program();

    //bind the model transform
    program.set_uniform_matrix4fv(
      model_uniform_loc.0,
      &transforms.get_model_transform(&position.0),
    );

    //bind the view transform
    program.set_uniform_matrix4fv(view_uniform_loc.0, &transforms.get_view_transform());

    //bind the projection transform
    program.set_uniform_matrix4fv(
      projection_uniform_loc.0,
      transforms.get_projection_transform().as_matrix(),
    );

    texture.bind(gl);
    
    vao.bind();
    unsafe {
      gl.DrawArrays(TRIANGLES, 0, 36);
    }
    vao.unbind();

  }
  Ok(())
}
