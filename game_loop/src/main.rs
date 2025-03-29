mod update;
mod user_interface;

use engine::{
  input::user_inputs::{FrameInputs, KeyAction, Keybinds},
  math::Transforms,
  renderer::{
    camera::Camera,
    sdl2_helpers::{PhysicalPosition, Window},
    Renderer,
  },
  systems::{register_components, register_resources, spawn_dummy, spawn_enviroment, spawn_player},
  time::ServerTime,
  windowing::create_window,
};
use nina::world::World;
use sdl2::{event::Event, keyboard::Keycode};
use std::sync::Arc;
use update::update;

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

  let (canvas, mut event_pump,) = create_window();

  //Create the camera
  let mut camera = Camera::default();
  let transforms = Transforms::from(canvas.window().inner_size(),);
  camera.update_pv(&transforms,);

  //Spawn the renderer
  let mut renderer = pollster::block_on(Renderer::new(Arc::new(canvas,),),);

  //Spawn the player
  spawn_player(&mut world, "warrior", 1, &mut renderer,);

  //Spawn the ground
  spawn_enviroment(&mut world, "ground", &mut renderer,);

  //Spawn dummies
  spawn_dummy(&mut world, [3.0, 0.0, -3.0,], &mut renderer,);
  spawn_dummy(&mut world, [5.0, 0.0, 0.0,], &mut renderer,);

  //Add the resources to world
  world.add_resource(camera,);
  world.add_resource(transforms,);

  // TODO: Copy the example to implement the function that takes those things as
  // arguments.
  // See if there is a way around having to multithread.
  // From this example https://github.com/awwsmm/hello-rust-sdl2-wasm/blob/master/src/main.rs

  // Set clear the canvas
  // TODO: Does canvas clear need to happen like in the sdl2 document example:https://docs.rs/sdl2/latest/sdl2/index.html

  // Create the mouse input just outside the loop.
  // This basically needs to constantly update
  // TODO: This does not feel like the best way to do this
  // TODO: But the fact you cant't borrow the event pump inside the loop is
  // problematic
  let mut mouse_pos = PhysicalPosition::new(
    event_pump.mouse_state().x() as f64,
    event_pump.mouse_state().y() as f64,
  );

  'game: loop {
    for event in event_pump.poll_iter() {
      match event {
        // Code to exit the game
        Event::Quit { .. }
        | Event::KeyDown {
          keycode: Some(Keycode::Escape,),
          ..
        } => break 'game,
        // Handle mouse movements
        // Don't think it will come up but the "relative" coordinates are really displacement
        Event::MouseMotion { x, y, .. } => {
          let dimensions = renderer.window().inner_size();

          mouse_pos = PhysicalPosition::from_screen_coords(x, y, dimensions,);
        }
        // TODO: Unsure if using the mouse_pos variable or these directly is better. These directky
        // most likely since they will be the most up to date
        Event::MouseButtonDown {
          mouse_btn, x, y, ..
        } => {
          // Create the mouse position
          let dimensions = renderer.window().inner_size();
          let mouse_pos = PhysicalPosition::from_screen_coords(x, y, dimensions,);

          // Add the input
          let keybinds = world.get_resource::<Keybinds>();
          if let Ok(input,) = keybinds.mouse_input(&world, &mouse_pos, &mouse_btn,) {
            let inputs = world.get_resource_mut::<FrameInputs>();
            inputs.push(input,)
          }
        }

        // Handle keypresses by generating a frame input.
        // TODO: Maybe make this into a self contained function both for documentation perposes and
        // also to reduce clutter
        Event::KeyDown {
          keycode: Some(key,),
          ..
        } => {
          // Generate an input for the keypress
          // TODO: Would it be better to get the position via
          // `event_pump.mouse_state().x()` insteaad of constantly tracking it? Could
          // maybe implement a function or trait on the pump to make a direct query for
          // mouse positon in NDC possible?
          let keybinds = world.get_resource::<Keybinds>();
          let input = keybinds.key_input(&world, &mouse_pos, key, KeyAction::Press,);
          match input {
            // If the input is valid add it to the frame inputs
            Ok(input,) => world.get_resource_mut::<FrameInputs>().push(input,),
            Err(_,) => {
              // TODO: Could print the error message to the a debug file or
              // something for debugging but not urgent
            }
          }
        }
        Event::KeyUp {
          keycode: Some(key,),
          ..
        } => {
          // Generate an input for the keypress
          // TODO: Would it be better to get the position via
          // `event_pump.mouse_state().x()` insteaad of constantly tracking it? Could
          // maybe implement a function or trait on the pump to make a direct query for
          // mouse positon in NDC possible?
          let keybinds = world.get_resource::<Keybinds>();
          let input = keybinds.key_input(&world, &mouse_pos, key, KeyAction::Release,);
          match input {
            // If the input is valid add it to the frame inputs
            Ok(input,) => world.get_resource_mut::<FrameInputs>().push(input,),
            Err(_,) => {
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
      // Update the server time
      world
        .get_resource_mut::<ServerTime>()
        .decrement_seconds_since_update();
    }

    // TODO: I *think* this needs to be a different server time than before the
    // update loop since it needs to reflect how long the update took
    let server_time = world.get_resource::<ServerTime>();
    if server_time.should_render() {
      renderer.update(&world,);
      renderer.render().unwrap();
      let server_time = world.get_resource_mut::<ServerTime>();
      server_time.decrement_seconds_since_render()
    }
  }

  // .run(move |event, target| match event {
  //   Event::AboutToWait => {
  //     //UPDATE
  //     {
  //       let server_time = world.get_resource_mut::<ServerTime>();
  //       server_time.tick();
  //     }

  //     if world.get_resource_mut::<ServerTime>().should_update() {
  //       update(&mut world,);

  //       //Update the delta timer
  //       let server_time = world.get_resource_mut::<ServerTime>();
  //       server_time.decrement_seconds_since_update()
  //     }

  //     //RENDER
  //     if world.get_resource_mut::<ServerTime>().should_render() {
  //       renderer.window().request_redraw();
  //     }
  //   }
  //   Event::WindowEvent {
  //     event, window_id, ..
  //   } => match event {
  //     WindowEvent::CursorMoved { mut position, .. } => {
  //       let dimensions = renderer.window().inner_size();

  //       //Convert the mouse to ndc coords
  //       position.x = 2.0 * position.x as f64 / dimensions.width as f64 - 1.0;
  // //range [-1,1]       position.y = 1.0 - (2.0 * position.y as f64) /
  // dimensions.height as f64; //range [-1,1]

  //       mouse_pos = Some(position,);
  //     }
  //     WindowEvent::MouseInput { button, .. } => {
  //       if let Some(mouse_pos,) = mouse_pos {
  //         let keybinds = world.get_resource::<Keybinds>();
  //         if let Ok(input,) = keybinds.mouse_input(&world, &mouse_pos,
  // &button,) {           let inputs =
  // world.get_resource_mut::<FrameInputs>();           inputs.push(input,)
  //         }
  //       }
  //     }
  //     WindowEvent::KeyboardInput {
  //       event: KeyEvent {
  //         physical_key: key, ..
  //       },
  //       ..
  //     } => {
  //       if key == PhysicalKey::Code(KeyCode::Escape,) {
  //         target.exit();
  //       }
  //       let keybinds = world.get_resource::<Keybinds>();
  //       if let Some(mouse_pos,) = mouse_pos {
  //         if let Ok(input,) = keybinds.key_input(&world, &mouse_pos, key,) {
  //           let inputs = world.get_resource_mut::<FrameInputs>();
  //           inputs.push(input,);
  //         }
  //       }
  //     }
  //     WindowEvent::RedrawRequested => {
  //       if window_id == renderer.window().id() {
  //         renderer.update(&world,);
  //         renderer.render().unwrap();
  //         let server_time = world.get_resource_mut::<ServerTime>();
  //         server_time.decrement_seconds_since_render()
  //       }
  //     }
  //     WindowEvent::Resized(size,) => {
  //       renderer.resize(size,);
  //       let transforms = world.get_resource_mut::<Transforms>();
  //       *transforms = Transforms::from(size,);
  //     }
  //     WindowEvent::CloseRequested => target.exit(),
  //     _ => {}
  //   },
  //   _ => {}
  // },)
  // .unwrap();
}
