use crate::{
  ecs::{
    component_lib::{
      ModelUniformLocation, Position,
    },
    World, world_resources::ShaderPrograms,
  },
  math::{Transforms, Vec3},
  view::{{SkinnedMesh,StaticMesh}, render_gl::Program}
};
use eyre::Result;
use gl::{Gl, COLOR_BUFFER_BIT, DEPTH_BUFFER_BIT, TRIANGLES, STENCIL_BUFFER_BIT, ALWAYS, NOTEQUAL, DEPTH_TEST};
use glm::lerp;

//I need to find a way to make the render positions consistent accross the sub functions that use it
//honestly, could just set it in a separate server side system that updates with the render loop
pub fn render(world:&World, interpolation_factor:f64) -> Result<()> {
  let gl = world.immut_get_resource::<Gl>().unwrap();
  let programs = world.immut_get_resource::<ShaderPrograms>().unwrap();

  unsafe { 
    gl.ClearColor(0.1, 0.1, 0.1, 1.0);
    gl.Clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT | STENCIL_BUFFER_BIT) 
  }

  //set uniforms
  programs.set_highlight_uniforms(world);
  programs.set_normal_uniforms(world);

  unsafe { gl.StencilMask(0x00) };
  programs.normal.use_program();
  render_static_geometry(&world,&programs.normal)?;
  
  //First Render Pass
  unsafe{    
    gl.StencilFunc(ALWAYS, 1, 0xFF);
    gl.StencilMask(0xFF);
  }
  
  render_skinned_meshes(&world,&programs.normal, interpolation_factor)?;

  unsafe{
    gl.StencilFunc(NOTEQUAL, 1, 0xFF);
    gl.StencilMask(0x00);
    gl.Disable(DEPTH_TEST);
  }

  programs.highlight.use_program();
  render_special_outlines(&world,&programs.highlight, interpolation_factor)?;
  
  unsafe{  
    gl.StencilMask(0xFF);
    gl.StencilFunc(ALWAYS, 1, 0xFF);
    gl.Enable(DEPTH_TEST);
  }

  Ok(())
}

// pub fn render_fog(){}
// pub fn render_shadows(){}
fn render_static_geometry(world:&World,program:&Program) -> Result<()> {
  let gl = world.immut_get_resource::<Gl>().unwrap();
  let transforms = world.immut_get_resource::<Transforms>().unwrap();
  let model_uniform_loc = world.immut_get_resource::<ModelUniformLocation>().unwrap();
  
  let mut query = world.query();

  let entities = query
    .with_component::<StaticMesh>()?
    .with_component::<Position>()?
    .run_entity();

    for entity in entities {
      let position = entity.immut_get_component::<Position>()?;
      //this is smoother but starts jerking around at high speeds
      let render_position:Vec3 = position.tick_start;
  
      let mesh = entity.immut_get_component::<StaticMesh>()?;
      let texture = &mesh.texture;
      let vao = &mesh.vao;
  
      //bind the model transform
      program.set_uniform_matrix4fv(
        model_uniform_loc.0,
        &transforms.get_model_transform(&render_position,1.0)
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

fn render_skinned_meshes(world:&World, program:&Program, interpolation_factor:f64) -> Result<()>{
  let gl = world.immut_get_resource::<Gl>().unwrap();
  let transforms = world.immut_get_resource::<Transforms>().unwrap();
  let model_uniform_loc = world.immut_get_resource::<ModelUniformLocation>().unwrap();

  let mut query = world.query();

  let entities = query
    .with_component::<SkinnedMesh>()?
    .with_component::<Position>()?
    .run_entity();

  for entity in entities {
    let position = entity.immut_get_component::<Position>()?;
    //this is smoother but starts jerking around at high speeds
    let render_position:Vec3 = lerp(&position.tick_start,&position.tick_end,interpolation_factor as f32);

    let mesh = entity.immut_get_component::<SkinnedMesh>()?;
    let texture = &mesh.texture;
    let vao = &mesh.vao;

    texture.bind(gl);
    vao.bind();

    //bind the model transform
    program.set_uniform_matrix4fv(
      model_uniform_loc.0,
      &transforms.get_model_transform(&render_position,1.0)
    );

    unsafe {
      gl.DrawArrays(TRIANGLES, 0, 36);
    }
    vao.unbind();
  }

  Ok(())
}
// pub fn render_outlines(){}
// pub fn render_decals(){}
fn render_special_outlines(world:&World, program:&Program, interpolation_factor:f64) -> Result<()> {
  let gl = world.immut_get_resource::<Gl>().unwrap();
  let transforms = world.immut_get_resource::<Transforms>().unwrap();
  let model_uniform_loc = world.immut_get_resource::<ModelUniformLocation>().unwrap();

  let mut query = world.query();

  let entities = query
    .with_component::<SkinnedMesh>()?
    .with_component::<Position>()?
    // .with_component::<Selected>()?
    .run_entity();

  for entity in entities {
    let position = entity.immut_get_component::<Position>()?;
    //this is smoother but starts jerking around at high speeds
    let render_position:Vec3 = lerp(&position.tick_start,&position.tick_end,interpolation_factor as f32);

    let mesh = entity.immut_get_component::<SkinnedMesh>()?;
    let texture = &mesh.texture;
    let vao = &mesh.vao;

    texture.bind(gl);
    vao.bind();

    //bind the model transform
    program.set_uniform_matrix4fv(
      model_uniform_loc.0,
      &transforms.get_model_transform(&render_position,1.1)
    );

    unsafe {
      gl.DrawArrays(TRIANGLES, 0, 36);
    }
    vao.unbind();
  }

  Ok(())
}
// pub fn render_particles(){}
//health bars, status text, etc
// pub fn render_indicators(){}
// pub fn render_hud(){}