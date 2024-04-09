// use sdl2::{pixels::Color, render::Canvas, video::Window};
use winit::{
  dpi::{LogicalSize, Size},
  event::{Event, WindowEvent},
  event_loop::EventLoop,
  window::{Window, WindowBuilder},
};

// ///Creates and returns an [`sdl2`] [`Canvas`]<[`Window`]>.
// pub fn create_sdl2_window() -> Canvas<Window> {
//   //Create the SDL window and canvas
//   let sdl_context = sdl2::init().unwrap();
//   let video_subsystem = sdl_context.video().unwrap();
//   let window = video_subsystem.window("Deux", 1280, 720).position_centered().build().unwrap();
//   let mut canvas = window.into_canvas().build().unwrap();

//   //Set the clear color to black and switch the framebuffers
//   canvas.set_draw_color(Color::RGB(0, 0, 0));
//   canvas.clear();
//   canvas.present();
//   canvas
// }

///Creates and returns a [winit](https://docs.rs/winit/latest/winit/index.html) [`Window`].
pub fn create_window() -> (Window, EventLoop<()>) {
  //Create the eventloop and window
  let eventloop = EventLoop::new().unwrap();

  //Create the window
  let window = WindowBuilder::new().with_inner_size(LogicalSize::new(1280, 720)).build(&eventloop).unwrap();

  //Set the loop polling state
  eventloop.set_control_flow(winit::event_loop::ControlFlow::Poll);

  (window, eventloop)
}
