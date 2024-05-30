use engine::{
  event::GameEventQueue,
  input::user_inputs::FrameInputs,
  systems::{casting, combat, execute_scripts, movement, process_inputs}
};
use nina::world::World;

pub fn update(world:&mut World) {
  process_inputs(world);
  execute_scripts(world);
  movement(world);
  casting(world);
  combat(world);

  let frame_inputs = world.get_resource_mut::<FrameInputs>();
  frame_inputs.clear();
  let events = world.get_resource_mut::<GameEventQueue>();
  events.clear();
  events.move_pending();
}
