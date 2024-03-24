use crate::{
  arena::Grid,
  ecs::{
    world_resources::{DbgShaderProgram, DebugElements, ScreenDimensions, Selected, ShaderPrograms},
    World,
  },
  event::GameEventQueue,
  input::user_inputs::{FrameInputs, Keybinds},
  math::{MouseRay, Transforms},
  time::ServerTime,
  view::window::{create_gl, create_window},
};
use glfw::{Glfw, Window, WindowEvent};
use mlua::Lua;
use std::rc::Rc;
use std::sync::mpsc::Receiver;

pub fn register_resources(world: &mut World) -> (Glfw, Window, Receiver<(f64, WindowEvent)>) {
  let server_time = ServerTime::new();
  //make a settings file and load in from there
  let screen_dimensions = ScreenDimensions::new(1280, 720);

  // let grid = load_grid("5v5", "json").unwrap();
  let grid = Grid::new(100, 100, 1.0).unwrap();

  let lua = Rc::new(Lua::new());
  lua.globals().set("grid", grid).unwrap();

  let keybinds = Keybinds::default();
  let (glfw, mut window, events) = create_window(&screen_dimensions);
  let gl = create_gl(&mut window);

  //Create the shader programs
  let programs = ShaderPrograms::new(&gl).unwrap();
  let dbg_program = DbgShaderProgram::new(&gl);

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
    .add_resource(lua)
    //Add Keybinds
    .add_resource(keybinds)
    //Add Gl
    .add_resource(gl)
    //add the shader programs as a resource
    .add_resource(programs)
    .add_resource(dbg_program);

  (glfw, window, events)
}
