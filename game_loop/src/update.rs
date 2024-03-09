use engine::{
  ecs::{
    systems::{combat, execute_scripts, movement, process_inputs},
    World,
  },
  event::GameEventQueue,
  input::user_inputs::FrameInputs,
};

pub fn update(world: &mut World) {
  process_inputs(world);
  execute_scripts(world);
  movement(world);
  combat(world);

  let mut frame_inputs = world.get_resource_mut::<FrameInputs>().unwrap();
  frame_inputs.clear();
  let mut events = world.get_resource_mut::<GameEventQueue>().unwrap();
  events.clear();
  events.move_pending();
}
