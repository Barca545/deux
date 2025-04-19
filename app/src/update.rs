use crate::systems::{inputs::process_movement_inputs, movement::movement};
use inputs::frame_inputs::FrameInputs;
use nina::world::World;

pub fn update(world: &mut World,) {
  process_movement_inputs(world,);
  movement(world,);

  // Clean up the frame's inputs
  world.get_resource_mut::<FrameInputs>().end_frame();
}
