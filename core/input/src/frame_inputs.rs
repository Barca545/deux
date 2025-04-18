use sdl2::mouse::MouseState;

use crate::keybinds::{ButtonAction, Input, InputType};
use std::collections::{HashMap, HashSet};

// TODO: I believe there are better options than a hashmap for faster look up

pub struct FrameInputs {
  // TODO: I am not 100% sure if this is how I want to track this property
  /// All \[presssed\] [`InputType`]s retained from the previous frame. Used to
  /// track any `InputType` being held down across frames.
  holdover: HashSet<InputType,>,
  /// All [`Input`]s pressed and released during the current frame. Resets at
  /// the end of each frame.
  frame: HashMap<InputType, Vec<Input,>,>,
}

impl FrameInputs {
  pub fn new() -> Self {
    FrameInputs {
      holdover: HashSet::new(),
      frame: HashMap::new(),
    }
  }

  /// Record a new [`Input`] in the current frame.
  pub fn insert(&mut self, input: Input,) {
    self
      .frame
      .entry(input.ty,)
      .and_modify(|inputs| inputs.push(input,),)
      .or_insert(vec![input],);
  }

  /// Insert a mouse input.
  pub fn insert_mouse(&self, mouse: MouseState,) -> Input {
    Input {
      ty: InputType::Mouse,
      mouse,
      action: ButtonAction::None,
    }
  }

  /// Return the number of times a given ([`Input`], [`ButtonAction`])
  /// combination occured in a frame.
  pub fn number_of(&self, state: (InputType, ButtonAction,),) -> u8 {
    self
      .frame
      .get(&state.0,)
      .unwrap()
      .iter()
      .fold(0, |num, input| match (input.ty, input.action,) == state {
        true => num + 1,
        false => num + 0,
      },)
  }

  /// Returns true if the [`InputType`] was held at the beginning of the frame.
  pub fn was_held(&self, ty: &InputType,) -> bool {
    self.holdover.contains(ty,)
  }

  /// Returns true if the [`InputType`] is pressed at the end of the frame.
  pub fn is_pressed(&self, ty: &InputType,) -> bool {
    self.holdover.contains(ty,)
      || match self.frame.get(ty,) {
        Some(inputs,) => inputs.last().unwrap().action == ButtonAction::Press,
        None => false,
      }
  }

  /// Move any held buttons into [`FrameInputs::holdover`] and purge
  /// [`FrameInputs::frame`].
  pub fn end_frame(&mut self,) {
    // If any buttons are held down, hold them over
    // Buttons are held if the last entry in their vec is ButtonAction::Press
    // This also serves to clear out the map
    for (ty, inputs,) in self.frame.drain() {
      match inputs.last().unwrap().action {
        ButtonAction::Press => {
          self.holdover.insert(ty,);
        }
        ButtonAction::Release => {
          self.holdover.remove(&ty,);
        }
        ButtonAction::None => {}
      }
    }
    // TODO: A more efficient way to clear might be to set them to vec![]
    // because odds are all the elements will be reused during the game so
    // reconstructing the map with new insertions is a waste
  }
}

#[cfg(test)]
mod test {
  use super::FrameInputs;
  use crate::keybinds::{ButtonAction, Input, InputType};
  use sdl2::mouse::MouseState;

  #[test]
  fn count_number_inputs() {
    let input = Input {
      ty: InputType::MoveDown,
      mouse: MouseState::from_sdl_state(32,),
      action: ButtonAction::None,
    };

    let mut inputs = FrameInputs::new();

    inputs.insert(input,);
    inputs.insert(input,);
    inputs.insert(input,);
    inputs.insert(input,);

    let num_1 = inputs.number_of((InputType::MoveDown, ButtonAction::None,),);
    let num_2 = inputs.number_of((InputType::MoveDown, ButtonAction::Press,),);

    assert_eq!(num_1, 4);
    assert_eq!(num_2, 0);
  }
}
