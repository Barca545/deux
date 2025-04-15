use crate::sdl2_utils::{PhysicalSize, BLACK};
use sdl2::{self, render::Canvas, EventPump};
use std::{
  ops::{Deref, DerefMut},
  sync::Arc,
};

type WindowInner = Arc<sdl2::video::Window,>;
pub struct Window {
  pub inner: WindowInner,
  pub event_pump: EventPump,
}

// impl Deref for Window {
//   type Target = WindowInner;

//   fn deref(&self,) -> &Self::Target {
//     &self.inner
//   }
// }

// impl DerefMut for Window {
//   fn deref_mut(&mut self,) -> &mut Self::Target {
//     &mut self.inner
//   }
// }

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
      .window("Deux 2 The Second", size.width, size.height,)
      .position_centered()
      .build()
      .unwrap();

    // Convert the window into a canvas (this is what you can actually draw on)
    // TODO: Do I actually need the the canvas? or do I just need the window?
    // let mut canvas = window.into_canvas().build().unwrap();
    // canvas.set_draw_color(BLACK,);
    // canvas.clear();
    // canvas.present();

    // Create the event pump
    let event_pump = sdl2_context.event_pump().unwrap();

    // (canvas, event_pump,)
    Window {
      inner: Arc::new(window,),
      event_pump,
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

// /// Creates and returns a [`Window`].
// // pub fn create_window() -> (Canvas<sdl2::video::Window,>, EventPump,) {
// pub fn create_window() -> (Window, EventPump,) {
//   // Create the window and canvas
//   let sdl2_context = sdl2::init().unwrap();
//   let video_subsystem = sdl2_context.video().unwrap();

//   // TODO: This actually might be the kind of thing that could be a static
//   // It exists the whole program, most things just need to reference it.
//   let size = PhysicalSize::new(1280, 720,);

//   let window = video_subsystem
//     .window("Deux 2 The Second", size.width, size.height,)
//     .position_centered()
//     .build()
//     .unwrap();

//   // Convert the window into a canvas (this is what you can actually draw on)
//   // TODO: Do I actually need the the canvas? or do I just need the window?
//   // let mut canvas = window.into_canvas().build().unwrap();
//   // canvas.set_draw_color(BLACK,);
//   // canvas.clear();
//   // canvas.present();

//   // Create the event pump
//   let event_pump = sdl2_context.event_pump().unwrap();

//   // (canvas, event_pump,)
//   (Window(Arc::new(window,),), event_pump,)
// }
