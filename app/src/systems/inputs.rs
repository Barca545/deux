use game_data::player_movement::PlayerMovement;
use inputs::{
  frame_inputs::FrameInputs,
  keybinds::{ButtonAction, InputType},
};
use nina::world::World;

// TODO: This only works for one player, hanges will need to be made once
// multiplayer is being worked on
pub fn process_movement_inputs(world: &mut World,) {
  let mut inputs = world.get_resource_mut::<FrameInputs>();
  // Only update if there are inputs
  let player_input_state = world.get_resource_mut::<PlayerMovement>();
  // TODO: See if it's faster with our without this check
  if inputs.contains(&InputType::movement(),) {
    inputs.process_inputs(&InputType::movement(), |input| {
      match (input.ty(), input.action(),) {
        // Handle key presses
        (InputType::MoveUp, ButtonAction::Press,) => player_input_state.up = true,
        (InputType::MoveDown, ButtonAction::Press,) => player_input_state.down = true,
        (InputType::MoveLeft, ButtonAction::Press,) => player_input_state.left = true,
        (InputType::MoveRight, ButtonAction::Press,) => player_input_state.right = true,
        // Handle key releases
        (InputType::MoveUp, ButtonAction::Release,) => player_input_state.up = false,
        (InputType::MoveDown, ButtonAction::Release,) => player_input_state.down = false,
        (InputType::MoveLeft, ButtonAction::Release,) => player_input_state.left = false,
        (InputType::MoveRight, ButtonAction::Release,) => player_input_state.right = false,
        _ => todo!(),
      }
    },);
  }
}
