use std::os::raw::c_void;

use gl::{Gl, DEPTH_TEST, KEEP, LESS, NOTEQUAL, REPLACE, STENCIL_TEST};
use glfw::{Context, RenderContext, Window};

use crate::{
  ecs::World,
  math::{Dimensions, Transforms},
};

// Refactor:
// -Should the stencil test stuff actually be set in the create GL function
// -Is there a reason I call gl context version in create gl? it's never used
// -Delete the create gl function from the create module
// -RenderContext is useful for getting the glfw struct but not the actual window

pub struct Renderer {
  //reference to the window
  //reference to transforms?
  //I kinda think it might be able to take in the above info in render
  ctx: RenderContext,

  gl: Gl,
}

impl Renderer {
  pub fn new(window: &mut Window) {
    let gl = create_gl(window);
  }

  //move code for resizing here
  pub fn resize(&self, world: &World, window: &Window) {
    //Update the dimension resource
    let (height, width) = window.get_size();
    let mut dimensions = world.get_resource_mut::<Dimensions>().unwrap();
    *dimensions = Dimensions::new(width, height);

    //Update the transforms resource
    let mut transforms = world.get_resource_mut::<Transforms>().unwrap();
    *transforms = Transforms::new(&dimensions.aspect);

    //Update the viewport
    unsafe { self.gl.Viewport(0, 0, width, height) }
  }

  //this should be the stuff in the create functions currently
  //also just create the meshes through the renderer
  // there's no reason for other systems to have access to gl
  pub fn init(&mut self) {}

  pub fn render(&self, world: &World, window: &mut Window) {
    //Render logic

    //Swap the frame buffer
    window.swap_buffers();
  }
}

fn create_gl(window: &mut Window) -> Gl {
  let _gl_context = window.get_context_version();
  let gl = Gl::load_with(&mut |s| window.get_proc_address(s) as *const c_void);
  unsafe {
    //Set clear color to Black
    gl.ClearColor(0.1, 0.1, 0.1, 1.0);
    gl.Enable(DEPTH_TEST);
    gl.DepthFunc(LESS);
    gl.Enable(STENCIL_TEST);
    gl.StencilFunc(NOTEQUAL, 1, 0xFF);
    gl.StencilOp(KEEP, KEEP, REPLACE);
  }
  gl
}
