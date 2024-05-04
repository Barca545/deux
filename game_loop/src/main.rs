// Refactor:
// -Switch to using FileType enum in the file system
// -Make window a resource?
// -Glfw.poll_events could probably go inside a function that goes inside the input system but confirm this doesn't have threading issues or anything
// -Update to cast abilities based on keyboard inputs.
// -Add a skillshot, AS steroid, blink, and point and click to test the ability scripting.
//  The point and click should have a burn effect.
// -Add death system and update queries to ignore dead entities
//  Issue is based on distance from screen, the entity closer to the user is selected first?
// -Move the resize window code into its own function and only run it if one of the events was a window resize
//  Window can be cloned and passed around

// Refactor - Rendering:
// -Experiment with putting Gl in an Rc
// -Meshes need to be a resource
// -Should be able to turn towards the direction of the mouse.
//  This probably necesitates adding a "facing" component like LoL has
// -World can be added as a resource. Maybe events can too?

// Refactor - Grid
// -Could probably replace the check for if position == new_position in the renderer once I add in some sort of movement state tracker
// -Consider moving to a slower tick rate LoL uses 30hz
// -Grid should load in from a JSON once I build the grid in the level editor
// -Grid might also need to be a resource. I'm unsure if other systems will need it
// -Dimensions should load from a settings file
// -Maybe I just pass it in directly to the system that handles inputs, or just pass a copy of the raw event pump and handle it there?

// Refactor - Movement:
// -Make the list of open/closed indexes a global in lua since it's constant throughout the game
// -Function to check the cell a given position is inside
// -Run an a* pathing script

// fn main() -> Result<()> {
//   //Configure the location of the asset folders
//   asset_config();

//   let mut world = World::new();

//   //Register the resources and create the window
//   let (mut glfw, mut window, events) = register_resources(&mut world);

//   //Register the components the game uses with the world
//   register_components(&mut world);

//   //Spawn the ground
//   spawn_enviroment(&mut world, "ground").unwrap();

//   //Spawn the players and dummies
//   spawn_player(&mut world, "warrior", 1)?;

//   spawn_dummy(&mut world, Vec3::new(3.0, 0.0, -3.0)).unwrap();
//   spawn_dummy(&mut world, Vec3::new(5.0, 0.0, 0.0)).unwrap();

//   //Main loop
//   while !window.should_close() {
//     //For some reason if this is not here I get a black screen
//     {
//       let mut server_time = world.get_resource_mut::<ServerTime>().unwrap();
//       server_time.tick();
//     }

//     glfw.poll_events();
//     for (_, event) in glfw::flush_messages(&events) {
//       match event {
//         glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
//         glfw::WindowEvent::Key(key, _, Action::Press, _) => {
//           let keybinds = world.get_resource::<Keybinds>().unwrap();
//           if let Ok(input) = keybinds.key_input(&world, &window, key) {
//             let mut inputs = world.get_resource_mut::<FrameInputs>().unwrap();
//             inputs.push(input)
//           }
//         }
//         glfw::WindowEvent::MouseButton(button, Action::Press, _) => {
//           let keybinds = world.get_resource::<Keybinds>().unwrap();
//           if let Ok(input) = keybinds.mouse_input(&world, &window, button) {
//             let mut inputs = world.get_resource_mut::<FrameInputs>().unwrap();
//             inputs.push(input)
//           }
//         }
//         _ => {}
//       }
//     }

//     let server_time = world.get_resource::<ServerTime>().unwrap().clone();

//     //Update
//     if server_time.should_update() == true {
//       update(&mut world);
//       //I think this is where I update the delta timer
//       let mut server_time = world.get_resource_mut::<ServerTime>().unwrap();
//       server_time.decrement_seconds_since_update()
//     }

//     //Render
//     //Can I clear the buffers before binding or do they need to be cleared after
//     // binding? Binding currently happens in their own functions.
//     if server_time.should_render() {
// //have some flag so it only runs if it was resized
// let (width, height) = window.get_size();
// {
//   let mut dimensions = world.get_resource_mut::<ScreenDimensions>().unwrap();
//   *dimensions = ScreenDimensions::new(width, height);
// }

// {
//   let dimensions = world.get_resource::<ScreenDimensions>().unwrap().clone();

//   let mut transforms = world.get_resource_mut::<Transforms>().unwrap();
//   *transforms = Transforms::new(&dimensions.aspect);

//   let gl = world.get_resource::<Gl>().unwrap();
//   unsafe { gl.Viewport(0, 0, width, height) }
// }

// //can maybe make the render function handle the swapbuffers
// render(&world);

// window.swap_buffers();
//       let mut server_time = world.get_resource_mut::<ServerTime>().unwrap();
//       server_time.decrement_seconds_since_render()
//     }
//   }
//   Ok(())
// }

use engine::{
  ecs::{
    systems::{register_components, register_resources, spawn_dummy, spawn_enviroment, spawn_player},
    World,
  },
  input::user_inputs::{FrameInputs, Keybinds},
  math::Transforms,
  time::ServerTime,
  view::{camera::Camera, Renderer},
  windowing::create_window,
};
use std::sync::Arc;
use winit::{
  event::{Event, KeyEvent, WindowEvent},
  keyboard::{KeyCode, PhysicalKey},
};
mod update;
use update::update;

// Refactor:
// -Re-add other systems
// -Move the input handling to its own mod maybe the windowing mod/file
// -Use a lazy static to get the config paths?
// -Move the event loop into separate functions?
// -Update the add resources method
// -Organize the inputs using helper functions and move the control flow into its own function
// -Renderer needs to interpolate I believe

fn main() {
  let mut world = World::new();
  register_components(&mut world);
  register_resources(&mut world);

  let (window, events) = create_window();

  //Create the camera
  let mut camera = Camera::default();
  let transforms = Transforms::from(window.inner_size());
  camera.update_pv(&transforms);

  //Spawn the renderer
  let mut renderer = pollster::block_on(Renderer::new(Arc::new(window)));

  //Spawn the player
  spawn_player(&mut world, "warrior", 1, &mut renderer);

  //Spawn the ground
  spawn_enviroment(&mut world, "ground", &mut renderer);

  //Spawn dummies
  spawn_dummy(&mut world, [3.0, 0.0, -3.0], &mut renderer);
  spawn_dummy(&mut world, [5.0, 0.0, 0.0], &mut renderer);

  //Add the resources to world
  world.add_resource(camera);
  world.add_resource(transforms);

  let mut mouse_pos = None;

  events
    .run(move |event, target| match event {
      Event::AboutToWait => {
        //UPDATE
        {
          let mut server_time = world.get_resource_mut::<ServerTime>().unwrap();
          server_time.tick();
        }

        if world.get_resource_mut::<ServerTime>().unwrap().should_update() {
          update(&mut world);

          //Update the delta timer
          let mut server_time = world.get_resource_mut::<ServerTime>().unwrap();
          server_time.decrement_seconds_since_update()
        }

        //RENDER
        if world.get_resource_mut::<ServerTime>().unwrap().should_render() {
          renderer.window().request_redraw();
        }
      }
      Event::WindowEvent { event, window_id, .. } => match event {
        WindowEvent::CursorMoved { mut position, .. } => {
          let dimensions = renderer.window().inner_size();

          //Convert the mouse to ndc coords
          position.x = 2.0 * position.x as f64 / dimensions.width as f64 - 1.0; //range [-1,1]
          position.y = 1.0 - (2.0 * position.y as f64) / dimensions.height as f64; //range [-1,1]

          mouse_pos = Some(position);
        }
        WindowEvent::MouseInput { button, .. } => {
          if let Some(mouse_pos) = mouse_pos {
            let keybinds = world.get_resource::<Keybinds>().unwrap();
            if let Ok(input) = keybinds.mouse_input(&world, &mouse_pos, &button) {
              let mut inputs = world.get_resource_mut::<FrameInputs>().unwrap();
              inputs.push(input)
            }
          }
        }
        WindowEvent::KeyboardInput {
          event: KeyEvent { physical_key: key, .. },
          ..
        } => {
          if key == PhysicalKey::Code(KeyCode::Escape) {
            target.exit();
          }
          let keybinds = world.get_resource::<Keybinds>().unwrap();
          if let Some(mouse_pos) = mouse_pos {
            if let Ok(input) = keybinds.key_input(&world, &mouse_pos, key) {
              let mut inputs = world.get_resource_mut::<FrameInputs>().unwrap();
              inputs.push(input);
            }
          }
        }
        WindowEvent::RedrawRequested => {
          if window_id == renderer.window().id() {
            renderer.update(&world);
            renderer.render().unwrap();
            let mut server_time = world.get_resource_mut::<ServerTime>().unwrap();
            server_time.decrement_seconds_since_render()
          }
        }
        WindowEvent::Resized(size) => {
          renderer.resize(size);
          let mut transforms = world.get_resource_mut::<Transforms>().unwrap();
          *transforms = Transforms::from(size);
        }
        WindowEvent::CloseRequested => target.exit(),
        _ => {}
      },
      _ => {}
    })
    .unwrap();
}
