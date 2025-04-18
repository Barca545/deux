use inputs::frame_inputs::FrameInputs;
use nina::world::World;

pub fn update(world: &mut World,) {
  // process_inputs(world,);
  // execute_scripts(world,);
  // movement(world,);
  // casting(world,);
  // combat(world,);

  // Clean up the frame events
  world.get_resource_mut::<FrameInputs>().end_frame();
  // let events = world.get_resource_mut::<GameEventQueue>();
  // events.clear();
  // events.move_pending();
}
