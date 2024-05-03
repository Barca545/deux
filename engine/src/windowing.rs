use winit::{
  dpi::LogicalSize,
  event_loop::EventLoop,
  window::{Window, WindowBuilder},
};

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
