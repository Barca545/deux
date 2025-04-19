use sdl2::mouse::MouseState;

use crate::keybinds::{ButtonAction, Input, InputType};
use std::{
  collections::{HashMap, HashSet},
  time::Instant,
};

// TODO: I believe there are better options than a hashmap for faster look up
#[derive(Debug,)]
/// A structure which tracks the [`Input`]s in a frame.
/// Also tracks any `Input`s which have been carried over from the previous
/// frame.
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
  pub fn insert_mouse(&self, mouse: MouseState, timestamp: Instant,) -> Input {
    Input {
      ty: InputType::Mouse,
      mouse,
      action: ButtonAction::None,
      timestamp,
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

  /// Returns true if the current frame contains any inputs with [`InputType`].
  pub fn contains(&self, types: &[InputType],) -> bool {
    for ty in types {
      if self.frame.contains_key(ty,) {
        return true;
      }
    }
    false
  }

  // TODO: Figuring out how to make this interact with releasing in a frame is
  // hard
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

  /// Locates the [`Input`] whose
  /// [`InputType`] and [Time](std::time::Instant) most closely match the
  /// parameters.
  pub fn find_closest(&self, ty: &InputType, time: &Instant,) -> Input {
    let inputs = self.frame.get(&ty,).unwrap();
    match inputs.binary_search_by_key(time, |probe| probe.timestamp,) {
      Ok(idx,) => inputs[idx],
      Err(idx,) => {
        let input_1 = inputs[idx - 1];
        let input_2 = match inputs.get(&idx + 1,) {
          Some(input_2,) => *input_2,
          None => return inputs[idx],
        };
        // Check the remainder of time - inputs[idx].time and return whichever
        // yields the smallest one.
        match *time - input_1.timestamp > *time - input_2.timestamp {
          true => input_2,
          false => input_1,
        }
      }
    }
  }

  /// Collects all [`Input`]s stored in [`FrameInputs`] which match the
  /// specified [`InputType`]s apply a callback function to each one.
  pub fn process_inputs<F,>(&self, types: &[InputType], mut f: F,)
  where
    F: FnMut(&Input,),
  {
    // Create the vec to iterate over
    let mut out_inputs = Vec::new();

    for ty in types {
      match self.frame.get(ty,) {
        Some(inputs,) => out_inputs.extend_from_slice(inputs,),
        None => {}
      }
    }

    for input in &out_inputs {
      f(input,)
    }
  }

  /// Get all the inputs of [`InputType`] which occured in the current frame.
  /// Returns `None` if no matching inputs occured.
  pub fn frame_get(&self, ty: &InputType,) -> Option<&Vec<Input,>,> {
    self.frame.get(ty,)
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
  use std::time::Instant;

  use super::FrameInputs;
  use crate::keybinds::{ButtonAction, Input, InputType};
  use sdl2::mouse::MouseState;

  #[test]
  fn count_number_inputs() {
    let input = Input {
      ty: InputType::MoveDown,
      mouse: MouseState::from_sdl_state(32,),
      action: ButtonAction::None,
      timestamp: Instant::now(),
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

  #[test]
  fn processing_inputs() {
    let input = Input {
      ty: InputType::MoveDown,
      mouse: MouseState::from_sdl_state(32,),
      action: ButtonAction::None,
      timestamp: Instant::now(),
    };

    let mut inputs = FrameInputs::new();

    inputs.insert(input,);
    inputs.insert(input,);
    inputs.insert(input,);
    inputs.insert(input,);

    let mut test = Vec::new();
    inputs.process_inputs(&InputType::movement(), |input| {
      test.push(input.clone(),);
    },);

    assert_eq!([input; 4], test.as_slice());
  }
}
