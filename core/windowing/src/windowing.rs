use crate::sdl2_utils::PhysicalSize;
use sdl2::{self, Sdl};
use std::sync::Arc;

type WindowInner = Arc<sdl2::video::Window,>;
pub struct Window {
  pub inner: WindowInner,
  pub sdl2: Sdl,
}

impl Window {
  /// Creates and returns a [`Window`].
  pub fn new() -> Self {
    // Create the window and canvas
    let sdl2_context = sdl2::init().unwrap();
    let video_subsystem = sdl2_context.video().unwrap();

    // TODO: This actually might be the kind of thing that could be a static
    // It exists the whole program, most things just need to reference it.
    let size = PhysicalSize::new(1280, 720,);

    let window = video_subsystem
      .window("Deux 2: The Second Part Two", size.width, size.height,)
      .position_centered()
      .build()
      .unwrap();

    Window {
      inner: Arc::new(window,),
      // event_pump,
      sdl2: sdl2_context,
    }
  }

  pub fn inner_size(&self,) -> PhysicalSize<u32,> {
    let size = self.inner.size();
    PhysicalSize {
      width: size.0,
      height: size.1,
    }
  }
}
