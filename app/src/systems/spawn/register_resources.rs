use game_data::{player_movement::PlayerMovement, DebugElements, Selected};
use inputs::{frame_inputs::FrameInputs, keybinds::Keybinds};
use nina::world::World;
use time::ServerTime;

// Refactor:
// -Need a settings file.
//  Keybinds should load in from settings file. update default/new to reflect
// that.

// TODO: This isn't registering this is actually *adding* a resource.
pub fn register_resources(world: &mut World,) {
  world
    .add_resource(Selected::NONE,)
    .add_resource(FrameInputs::new(),)
    .add_resource(ServerTime::new(),)
    .add_resource(DebugElements::new(false, false,),)
    .add_resource(Keybinds::default(),)
    .add_resource(PlayerMovement::new(),);
}
