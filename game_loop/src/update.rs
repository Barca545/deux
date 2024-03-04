use engine::{
  ecs::{
    systems::{combat, movement, process_inputs, update_target},
    World,
  },
  event::GameEventQueue,
  input::user_inputs::FrameInputs,
};

pub fn update(world: &mut World) {
  update_target(world);
  process_inputs(world);
  movement(world);
  combat(world);

  let mut frame_inputs = world.get_resource_mut::<FrameInputs>().unwrap();
  frame_inputs.clear();
  let mut events = world.get_resource_mut::<GameEventQueue>().unwrap();
  events.clear();
}