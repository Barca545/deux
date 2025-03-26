use crate::{
  data_lib::{DebugElements, Selected},
  event::GameEventQueue,
  input::user_inputs::{FrameInputs, Keybinds, PlayerInputs},
  math::MouseRay,
  time::ServerTime,
};
use nina::world::World;

// Refactor:
// -Need a settings file.
//  Dimension settings should load in from settings file.
//  Keybinds should load in from settings file. update default/new to reflect
// that.

// TODO: This isn't registering this is actually *adding* a resource.
pub fn register_resources(world:&mut World,) {
  // let grid = load_grid("5v5", "json").unwrap();
  // let grid = Grid::new(100, 100, 1.0,).unwrap();

  world
    .add_resource(Selected::NONE,)
    .add_resource(MouseRay::default(),)
    .add_resource(FrameInputs::new(),)
    .add_resource(ServerTime::new(),)
    .add_resource(DebugElements::new(false, false,),)
    .add_resource(GameEventQueue::new(),)
    //Add Keybinds
    .add_resource(Keybinds::default(),)
    // TODO: Merge the mouse ray stuff into the player inputs
    .add_resource(PlayerInputs::new(),);
}
