use crate::{
  arena::Grid,
  ecs::{
    world_resources::{DebugElements, Selected},
    World,
  },
  event::GameEventQueue,
  input::user_inputs::{FrameInputs, Keybinds},
  math::{Dimensions, MouseRay, Transforms},
  time::ServerTime,
};
use glfw::{Glfw, Window, WindowEvent};
use mlua::Lua;
use std::rc::Rc;
use std::sync::mpsc::Receiver;

// Refactor:
// -Honestly, this needs to be renamed and heavily refactored.
// -Need a settings file.
//  Dimension settings should load in from settings file.
//  Keybinds should load in from settings file.
// -Screen dimensions etc should not have to be in register resources, if anything they should be arguments

pub fn register_resources(world: &mut World) {
  let screen_dimensions = Dimensions::new(1280, 720);
  // let (glfw, mut window, events) = create_window(&screen_dimensions);
  // let gl = create_gl(&mut window);

  // let grid = load_grid("5v5", "json").unwrap();
  let grid = Grid::new(100, 100, 1.0).unwrap();

  let lua = Rc::new(Lua::new());
  lua.globals().set("grid", grid).unwrap();

  let keybinds = Keybinds::default();

  world
    .add_resource(screen_dimensions)
    // .add_resource(Transforms::new(&screen_dimensions.aspect))
    .add_resource(Selected::NONE)
    .add_resource(MouseRay::default())
    .add_resource(FrameInputs::new())
    .add_resource(ServerTime::new())
    .add_resource(DebugElements::new(false, false))
    .add_resource(GameEventQueue::new())
    //Initialize Lua
    .add_resource(lua)
    //Add Keybinds
    .add_resource(keybinds);
  //Add Gl
  // .add_resource(gl);

  // (glfw, window, events)
}
