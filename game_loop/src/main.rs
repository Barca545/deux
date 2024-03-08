mod update;

extern crate engine;
extern crate gl;
extern crate glfw;
extern crate nalgebra_glm as glm;

use engine::{
  config::asset_config,
  ecs::{
    systems::{register_components, register_resources, render, spawn_dummy, spawn_enviroment, spawn_player},
    world_resources::ScreenDimensions,
    World,
  },
  input::user_inputs::{FrameInputs, Keybinds},
  math::{Transforms, Vec3},
  time::ServerTime,
};
use gl::Gl;
use glfw::{Action, Context, Key};

use eyre::Result;

use update::update;
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

// Refactor - Rendering:
// -Experiment with putting Gl in an Rc
// -Meshes need to be a resource
// -Should be able to turn towards the direction of the mouse.
//  This probably necesitates adding a "facing" component like LoL has

// Refactor - Grid
// -Could probably replace the check for if position == new_position in the renderer once I add in some sort of movement state tracker
// -Consider moving to a slower tick rate LoL uses 30hz
// -Grid should load in from a JSON once I build the grid in the level editor
// -Grid might also need to be a resource. I'm unsure if other systems will need it
// -Dimensions should load from a settings file
// -Any way to make window a resource? Maybe I just pass it in directly to the system that handles inputs, or just pass a copy of the raw event pump and handle it there?

// Refactor - Movement:
// -Make the list of open/closed indexes a global in lua since it's constant throughout the game
// -Function to check the cell a given position is inside
// -Run an a* pathing script

fn main() -> Result<()> {
  //Configure the location of the asset folders
  asset_config();

  let mut world = World::new();

  //Register the resources and create the window
  let (mut glfw, mut window, events) = register_resources(&mut world);

  //Register the components the game uses with the world
  register_components(&mut world);

  //Spawn the ground
  spawn_enviroment(&mut world, "ground").unwrap();

  //Spawn the players and dummies
  spawn_player(&mut world, "warrior", 1)?;

  spawn_dummy(&mut world, Vec3::new(3.0, 0.0, -3.0)).unwrap();
  spawn_dummy(&mut world, Vec3::new(5.0, 0.0, 0.0)).unwrap();

  //Main loop
  while !window.should_close() {
    //For some reason if this is not here I get a black screen
    {
      let mut server_time = world.get_resource_mut::<ServerTime>().unwrap();
      server_time.tick();
    }

    glfw.poll_events();
    for (_, event) in glfw::flush_messages(&events) {
      match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        glfw::WindowEvent::Key(key, _, Action::Press, _) => {
          let keybinds = world.get_resource::<Keybinds>().unwrap();
          if let Ok(input) = keybinds.key_input(&world, &window, key) {
            let mut inputs = world.get_resource_mut::<FrameInputs>().unwrap();
            inputs.push(input)
          }
        }
        glfw::WindowEvent::MouseButton(button, Action::Press, _) => {
          let keybinds = world.get_resource::<Keybinds>().unwrap();
          if let Ok(input) = keybinds.mouse_input(&world, &window, button) {
            let mut inputs = world.get_resource_mut::<FrameInputs>().unwrap();
            inputs.push(input)
          }
        }
        _ => {}
      }
    }

    let server_time = world.get_resource::<ServerTime>().unwrap().clone();

    //Update
    if server_time.should_update() == true {
      update(&mut world);
      //I think this is where I update the delta timer
      let mut server_time = world.get_resource_mut::<ServerTime>().unwrap();
      server_time.decrement_seconds_since_update()
    }

    //Render
    //Can I clear the buffers before binding or do they need to be cleared after
    // binding? Binding currently happens in their own functions.
    if server_time.should_render() {
      //to do this window needs to be a resource
      //have some flag so it only runs if it was resized
      let (width, height) = window.get_size();
      {
        let mut dimensions = world.get_resource_mut::<ScreenDimensions>().unwrap();
        *dimensions = ScreenDimensions::new(width, height);
      }

      {
        let dimensions = world.get_resource::<ScreenDimensions>().unwrap().clone();

        let mut transforms = world.get_resource_mut::<Transforms>().unwrap();
        *transforms = Transforms::new(&dimensions.aspect);

        let gl = world.get_resource::<Gl>().unwrap();
        unsafe { gl.Viewport(0, 0, width, height) }
      }

      //can maybe make the render function handle the swapbuffers
      render(&world);

      window.swap_buffers();
      let mut server_time = world.get_resource_mut::<ServerTime>().unwrap();
      server_time.decrement_seconds_since_render()
    }
  }
  Ok(())
}
