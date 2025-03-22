use crate::view::sdl2_helpers::{PhysicalSize, BLACK};
use sdl2::{self, render::Canvas, video::Window, EventPump};

///Creates and returns a [winit](https://docs.rs/winit/latest/winit/index.html) [`Window`].
pub fn create_window() -> (Canvas<Window,>, EventPump,) {
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
  let mut canvas = window.into_canvas().build().unwrap();
  canvas.set_draw_color(BLACK,);
  canvas.clear();
  canvas.present();

  // Create the event pump
  let event_pump = sdl2_context.event_pump().unwrap();

  (canvas, event_pump,)
}
