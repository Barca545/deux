use crate::{
  ecs::{
    world_resources::{DebugElements, ShaderPrograms},
    World
  },
  view::render_gl::render_pass
};
use eyre::Result;
use gl::{Gl, ALWAYS, COLOR_BUFFER_BIT, DEPTH_BUFFER_BIT, DEPTH_TEST, NOTEQUAL, STENCIL_BUFFER_BIT};

//I need to find a way to make the render positions consistent accross the sub
// functions that use it honestly, could just set it in a separate server side
// system that updates with the render loop possibly find another way to get the
// interpolation factor do the interpolation factor * position as a system in
// beginning of the render loop and pass it down

pub fn render(world:&World, interpolation_factor:f64) -> Result<()> {
  let gl = world.immut_get_resource::<Gl>().unwrap();
  let programs = world.immut_get_resource::<ShaderPrograms>().unwrap();
  let debug_elements = world.immut_get_resource::<DebugElements>().unwrap();

  unsafe {
    gl.ClearColor(0.1, 0.1, 0.1, 1.0);
    gl.Clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT | STENCIL_BUFFER_BIT)
  }

  //set uniforms
  //do I need to do this every render loop or can I do it once?
  programs.set_highlight_uniforms(world);
  programs.set_normal_uniforms(world);

  unsafe { gl.StencilMask(0x00) };
  programs.normal.use_program(gl);
  render_pass::static_geometry(&world, &programs.normal)?;

  //First Render Pass
  unsafe {
    gl.StencilFunc(ALWAYS, 1, 0xFF);
    gl.StencilMask(0xFF);
  }
  render_pass::skinned_meshes(&world, &programs.normal, interpolation_factor)?;

  unsafe {
    gl.StencilFunc(NOTEQUAL, 1, 0xFF);
    gl.StencilMask(0x00);
    gl.Disable(DEPTH_TEST);
  }

  if debug_elements.aabb == true {
    render_pass::debug(&world, interpolation_factor)?;
  }

  programs.highlight.use_program(gl);
  render_pass::special_outlines(&world, &programs.highlight, interpolation_factor)?;

  unsafe {
    gl.StencilMask(0xFF);
    gl.StencilFunc(ALWAYS, 1, 0xFF);
    gl.Enable(DEPTH_TEST);
  }

  Ok(())
}

// pub fn render_fog(){}
// pub fn render_shadows(){}
// pub fn render_outlines(){}
// pub fn render_decals(){}
// pub fn render_particles(){}
// //health bars, status text, etc
// pub fn render_indicators(){}
// pub fn render_hud(){}
