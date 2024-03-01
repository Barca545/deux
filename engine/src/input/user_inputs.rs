use crate::math::MouseRay;
use std::fmt::Debug;

//Refactor:
// -Get input will have to be heavily refactored
// -Maybe each input type is a stored as a different struct that way they can be searched with turbofish
// -Alternatively just make a different fetch function for each input type
// -UserInput may not need to hold the mouse coordinates.
// -Replace the get input with the process input

#[derive(Debug, Clone, Copy)]
pub enum UserInput {
  //this should probably all be game events, events are literally like WindowEvents (user inputs)
  MouseClick(MouseRay),
  AbilityOnePress,
  AbilityTwoPress,
  AbilityThreePress,
  AbilityFourPress,
}

#[derive(Debug)]
pub struct FrameInputs {
  inputs: Vec<UserInput>,
}

impl FrameInputs {
  pub fn new() -> Self {
    FrameInputs { inputs: vec![] }
  }

  pub fn get_input(&self) -> Option<UserInput> {
    let filtered_input = self.inputs.clone().into_iter().find(|input| match input {
      UserInput::MouseClick(_) => true,
      _ => false,
    });
    filtered_input
  }

  ///Iterates over the [`UserInput`]s stored in the [`FrameInputs`] and applies a callback function.
  pub fn process_inputs<F>(&self, mut f: F)
  where
    F: FnMut(&UserInput),
  {
    for input in &self.inputs {
      f(input)
    }
  }

  ///Add a [`UserInput`] to the [`FrameInputs`].
  pub fn push(&mut self, event: UserInput) {
    self.inputs.push(event)
  }

  ///Run at the end of each tick to reset the input list.
  pub fn clear(&mut self) {
    //This needs to do something like not clear the ones still being held down?
    self.inputs.clear()
  }
}
