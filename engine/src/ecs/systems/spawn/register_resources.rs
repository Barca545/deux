use crate::{
  arena::Grid,
  ecs::{
    world_resources::{DebugElements, Selected},
    World,
  },
  event::GameEventQueue,
  input::user_inputs::{FrameInputs, Keybinds},
  math::MouseRay,
  time::ServerTime,
};
use mlua::Lua;
use std::rc::Rc;

// Refactor:
// -Honestly, this needs to be renamed and heavily refactored.
// -Need a settings file.
//  Dimension settings should load in from settings file.
//  Keybinds should load in from settings file. update default/new to reflect that.

pub fn register_resources(world: &mut World) {
  // let grid = load_grid("5v5", "json").unwrap();
  let grid = Grid::new(100, 100, 1.0).unwrap();

  let lua = Rc::new(Lua::new());
  lua.globals().set("grid", grid).unwrap();

  world
    .add_resource(Selected::NONE)
    .add_resource(MouseRay::default())
    .add_resource(FrameInputs::new())
    .add_resource(ServerTime::new())
    .add_resource(DebugElements::new(false, false))
    .add_resource(GameEventQueue::new())
    //Initialize Lua
    .add_resource(lua)
    //Add Keybinds
    .add_resource(Keybinds::default());
}
