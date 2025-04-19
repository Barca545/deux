use app::{
  render::render,
  systems::spawn::{
    register_components, register_resources, spawn_dummy, spawn_enviroment, spawn_player,
  },
  update,
};
use inputs::{
  frame_inputs::FrameInputs,
  keybinds::{ButtonAction, Keybinds},
};
use nina::world::World;
use renderer::{renderer::Renderer, scene::camera::Camera};
use sdl2::{
  event::{Event, WindowEvent},
  keyboard::Keycode,
  mouse::MouseState,
  EventPump,
};
use time::ServerTime;
// use update::update;
use windowing::{
  sdl2_utils::{PhysicalPosition, PhysicalSize},
  windowing::Window,
};

// Refactor:
// - Re-add other systems
// - Move the input handling to its own mod maybe the windowing mod/file
// - Use a OnceLock to get the config paths?
// - Renderer needs to interpolate I believe
// - Copy this example to make the keypressing stuff it's own file https://github.com/awwsmm/hello-rust-sdl2-wasm/blob/master/src/lib.rs
// - Organize the inputs using helper functions and move the control flow into
//   its own function
// - Look into the command pattern https://gameprogrammingpatterns.com/command.html
//   for inputs

fn main() {
  let mut world = World::new();
  // Register all the components and resources the game will need
  register_components(&mut world,);
  register_resources(&mut world,);

  let window = Window::new();

  //Create the camera
  let camera = Camera::new(
    window.inner_size().width as f32,
    window.inner_size().height as f32,
  );

  // Spawn the renderer
  let mut renderer = Renderer::new(&window,);
  renderer.add_opaque_pipeline("ModelShader",);

  // Spawn the player
  spawn_player(&mut world, "warrior", 1, &mut renderer,);

  // Spawn the ground
  spawn_enviroment(&mut world, "ground", &mut renderer,);

  // Spawn dummies
  spawn_dummy(&mut world, [3.0, 0.0, -3.0,], &mut renderer,);
  spawn_dummy(&mut world, [5.0, 0.0, 0.0,], &mut renderer,);

  // Add the resources to world
  world.add_resource(camera,);

  // TODO: Copy the example to implement the function that takes those things as
  // arguments.
  // See if there is a way around having to multithread.
  // From this example https://github.com/awwsmm/hello-rust-sdl2-wasm/blob/master/src/main.rs

  // Set clear the canvas
  // TODO: Does canvas clear need to happen like in the sdl2 document example:https://docs.rs/sdl2/latest/sdl2/index.html

  // Create the event pump
  let mut event_pump = window.sdl2.event_pump().unwrap();

  'game: loop {
    // TODO: Does this collect hit performance?
    for event in event_pump.poll_iter().collect::<Vec<_,>>() {
      match event {
        // Code to exit the game
        Event::Quit { .. }
        | Event::KeyDown {
          keycode: Some(Keycode::Escape,),
          ..
        } => break 'game,
        Event::Window {
          win_event: WindowEvent::Resized(width, height,),
          ..
        } => renderer.resize(PhysicalSize::new(width as u32, height as u32,),),
        // Record an event each time the mouse state is changed
        Event::MouseMotion { .. } | Event::MouseButtonDown { .. } => {
          let time = world.get_resource::<ServerTime>().get_current_tick();
          world
            .get_resource_mut::<FrameInputs>()
            .insert_mouse(event_pump.mouse_state(), time,);
        }
        // Generate an input for the keypress
        Event::KeyDown {
          keycode: Some(key,),
          ..
        } => {
          // TODO: Maybe make this into a self contained function both for documentation
          // perposes and also to reduce clutter?
          // If the input exists, record it
          let time = world.get_resource::<ServerTime>().get_current_tick();
          let keybinds = world.get_resource::<Keybinds>();
          match keybinds.build_input(&key, event_pump.mouse_state(), ButtonAction::Press, time,) {
            Some(input,) => {
              let mut inputs = world.get_resource_mut::<FrameInputs>();
              inputs.insert(input,);
            }
            None => {
              // TODO: Could print the error message to the a debug file or
              // something for debugging but not urgent
            }
          }
        }
        // Generate an input for the key release
        Event::KeyUp {
          keycode: Some(key,),
          ..
        } => {
          let keybinds = world.get_resource::<Keybinds>();
          // If the input exists, record it
          // Get the input's timestamp
          let time = world.get_resource::<ServerTime>().get_current_tick();
          match keybinds.build_input(&key, event_pump.mouse_state(), ButtonAction::Release, time,) {
            Some(input,) => {
              let mut inputs = world.get_resource_mut::<FrameInputs>();
              inputs.insert(input,);
            }
            None => {
              // TODO: Could print the error message to the a debug file or
              // something for debugging but not urgent
            }
          }
        }
        _ => {}
      }
    }

    // Tick the server before anything
    {
      let server_time = world.get_resource_mut::<ServerTime>();
      server_time.tick();
    }

    let server_time = world.get_resource::<ServerTime>();

    // Run update logic
    if server_time.should_update() {
      update(&mut world,);
      // Update the time since the last update
      world
        .get_resource_mut::<ServerTime>()
        .decrement_seconds_since_update();
    }

    // TODO: I *think* this needs to be a different server time than before the
    // update loop since it needs to reflect how long the update took
    let server_time = world.get_resource::<ServerTime>();
    if server_time.should_render() {
      // TODO: Use render system from the update mod
      render(&world, &mut renderer,);

      // Update the time since the last render
      world
        .get_resource_mut::<ServerTime>()
        .decrement_seconds_since_render()
    }
  }
}
