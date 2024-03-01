mod update;

extern crate engine;
extern crate gl;
extern crate glfw;
extern crate nalgebra_glm as glm;

use engine::{
  arena::Grid,
  config::asset_config,
  ecs::{
    systems::{register_components, render, spawn_dummy, spawn_enviroment, spawn_player, update_mouseray},
    world_resources::{DbgShaderProgram, DebugElements, ScreenDimensions, Selected, ShaderPrograms},
    World,
  },
  event::GameEventQueue,
  input::user_inputs::{FrameInputs, UserInput},
  math::{MouseRay, Transforms, Vec3},
  time::ServerTime,
  view::window::{create_gl, create_window},
};
use gl::Gl;
use glfw::{Action, Context, Key, MouseButton};
use mlua::Lua;
use update::update;
// Refactor:
// -Switch to using FileType enum in the file system
// -Update input system to be in one module
// -Make window a resource?
// -Glfw.poll_events could probably go inside a function that goes inside the input system but confirm this doesn't have threading issues or anything
// -Update to cast abilities based on keyboard inputs.
// -Add a skillshot, AS steroid, blink, and point and click to test the ability scripting.
//  The point and click should have a burn effect.
// -Add death system
// -Currently it seems like only one entity can be queried against which is why I can only select one dummy and ignore collisions with one dummy
//  Issue is based on distance from screen, the entity closer to the user is selected first?
// -Move the resize window code into its own function and only run it if one of the events was a window resize

// Refactor - Grid
// -Could probably replace the check for if position == new_position in the renderer once I add in some sort of movement state tracker
// -Consider moving to a slower tick rate LoL uses 30hz
// -Grid should load in from a JSON once I build the grid in the level editor
// -Grid might also need to be a resource. I'm unsure if other systems will need it
// -Dimensions should load from a settings file
// -Any way to make window a resource? Maybe I just pass it in directly to the system that handles inputs, or just pass a copy of the raw event pump and handle it there?

//use this wherever I handle the abilties to determine if they should check for a selection
//targeted abilities should only run if there is a selection
pub enum Ability {
  Targeted(String),
  Untargeted(String),
}

fn main() {
  //Configure the location of the asset folders
  asset_config();

  //could the thing where components are registered be part of world::default()
  let mut world = World::new();

  let server_time = ServerTime::new();
  //make a settings file and load in from there
  let screen_dimensions = ScreenDimensions::new(1280, 720);

  // let grid = load_grid("5v5", "json").unwrap();
  let grid = Grid::new(100, 100, 1.0).unwrap();

  let lua = Lua::new();
  lua.globals().set("grid", grid).unwrap();

  world
    .add_resource(screen_dimensions)
    .add_resource(Transforms::new(&screen_dimensions.aspect))
    .add_resource(Selected::NONE)
    .add_resource(MouseRay::default())
    .add_resource(FrameInputs::new())
    //add physics acceleration structure resource
    .add_resource(server_time)
    .add_resource(DebugElements::new(false, false))
    .add_resource(GameEventQueue::new())
    //Initialize Lua
    .add_resource(lua);

  let (mut glfw, mut window, events) = create_window(&world);
  let gl = create_gl(&mut window);

  //Add gl as a resource
  world.add_resource(gl.clone());

  //Create the shader programs
  let programs = ShaderPrograms::new(&world).unwrap();
  let dbg_program = DbgShaderProgram::new(&world);

  //add the programs as a resource
  world.add_resource(programs).add_resource(dbg_program);

  //Register the components the game uses with the world
  register_components(&mut world);

  //Spawn the ground
  spawn_enviroment(&mut world, "ground").unwrap();

  //Spawn the players and dummies
  spawn_player(&mut world, "warrior", 1).unwrap();

  spawn_dummy(&mut world, gl.clone(), Vec3::new(3.0, 0.0, -3.0));
  spawn_dummy(&mut world, gl.clone(), Vec3::new(5.0, 0.0, 0.0));
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
        glfw::WindowEvent::MouseButton(MouseButton::Button2, Action::Press, ..) => {
          let (x, y) = window.get_cursor_pos();
          let mouse_ray = update_mouseray(&world, x, y);
          let input = UserInput::MouseClick(mouse_ray);
          let mut inputs = world.get_resource_mut::<FrameInputs>().unwrap();
          inputs.push(input);
        }
        glfw::WindowEvent::Key(Key::Q, _, Action::Press, _) => {
          let input = UserInput::AbilityOnePress;
          let mut inputs = world.get_resource_mut::<FrameInputs>().unwrap();
          inputs.push(input);
        }
        glfw::WindowEvent::Key(Key::W, _, Action::Press, _) => {
          let input = UserInput::AbilityTwoPress;
          let mut inputs = world.get_resource_mut::<FrameInputs>().unwrap();
          inputs.push(input);
        }
        glfw::WindowEvent::Key(Key::E, _, Action::Press, _) => {
          let input = UserInput::AbilityThreePress;
          let mut inputs = world.get_resource_mut::<FrameInputs>().unwrap();
          inputs.push(input);
        }
        glfw::WindowEvent::Key(Key::R, _, Action::Press, _) => {
          let input = UserInput::AbilityFourPress;
          let mut inputs = world.get_resource_mut::<FrameInputs>().unwrap();
          inputs.push(input);
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
}
