use std::sync::mpsc::Receiver;

use gl::{Gl, DEPTH_TEST, KEEP, LESS, NOTEQUAL, REPLACE, STENCIL_TEST};
use glfw::{
  fail_on_errors, Context, Glfw, OpenGlProfileHint, Window, WindowEvent,
  WindowHint::{ContextVersionMajor, ContextVersionMinor, OpenGlProfile}
};

use crate::ecs::{world_resources::ScreenDimensions, World};

pub fn create_window(world:&World) -> (Glfw, Window, Receiver<(f64, WindowEvent)>) {
  let mut glfw = glfw::init(fail_on_errors!()).unwrap();
  glfw.window_hint(ContextVersionMajor(3));
  glfw.window_hint(ContextVersionMinor(3));
  glfw.window_hint(OpenGlProfile(OpenGlProfileHint::Core));

  let screen_dimensions = world.immut_get_resource::<ScreenDimensions>().unwrap();

  let (mut window, events) = glfw
    .create_window(
      screen_dimensions.width as u32,
      screen_dimensions.height as u32,
      "Project: Deux",
      glfw::WindowMode::Windowed
    )
    .expect("Failed to create GLFW window.");
  window.make_current();
  window.set_all_polling(true);

  (glfw, window, events)
}

pub fn create_gl(window:&mut Window) -> Gl {
  let _gl_context = window.get_context_version();
  let gl = Gl::load_with(&mut |s| window.get_proc_address(s) as *const std::os::raw::c_void);
  unsafe {
    gl.Enable(DEPTH_TEST);
    gl.DepthFunc(LESS);
    gl.Enable(STENCIL_TEST);
    gl.StencilFunc(NOTEQUAL, 1, 0xFF);
    gl.StencilOp(KEEP, KEEP, REPLACE);
  }
  gl
}