use crate::math::Dimensions;
use gl::{Gl, DEPTH_TEST, KEEP, LESS, NOTEQUAL, REPLACE, STENCIL_TEST};
use glfw::{
  fail_on_errors, Context, Glfw, OpenGlProfileHint, Window, WindowEvent,
  WindowHint::{ContextVersionMajor, ContextVersionMinor, OpenGlProfile},
  WindowMode,
};
use std::sync::mpsc::Receiver;

pub fn create_window(screen_dimensions: &Dimensions) -> (Glfw, Window, Receiver<(f64, WindowEvent)>) {
  let mut glfw = glfw::init(fail_on_errors!()).unwrap();
  glfw.window_hint(ContextVersionMajor(3));
  glfw.window_hint(ContextVersionMinor(3));
  glfw.window_hint(OpenGlProfile(OpenGlProfileHint::Core));

  let (mut window, events) = glfw
    .create_window(
      screen_dimensions.width as u32,
      screen_dimensions.height as u32,
      "Project: Deux",
      WindowMode::Windowed,
    )
    .expect("Failed to create GLFW window.");
  window.make_current();
  window.set_all_polling(true);

  (glfw, window, events)
}

pub fn create_gl(window: &mut Window) -> Gl {
  let _gl_context = window.get_context_version();
  let gl = Gl::load_with(&mut |s| window.get_proc_address(s) as *const std::os::raw::c_void);
  unsafe {
    //set clear color here or in the renderer
    gl.ClearColor(0.1, 0.1, 0.1, 1.0);
    gl.Enable(DEPTH_TEST);
    gl.DepthFunc(LESS);
    gl.Enable(STENCIL_TEST);
    gl.StencilFunc(NOTEQUAL, 1, 0xFF);
    gl.StencilOp(KEEP, KEEP, REPLACE);
  }
  gl
}
