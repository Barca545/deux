use std::fmt::Debug;
use crate::math::MouseRay;

//Refactor:
// -Get input will have to be heavily refactored
// -Maybe each input type is a stored as a different struct that way they can be searched with turbofish
// -Alternatively just make a different fetch function for each input type
// -UserInput may not need to hold the mouse coordinates.

//this is lazy, probably shouldn't be public but whatever
#[derive(Debug)]
pub struct MousePosition {
  pub x:f64,
  pub y:f64
}

#[derive(Debug,Clone, Copy)]
pub enum UserInput {
  MoveCameraUp,
  MoveCameraDown,
  MoveCameraRight,
  MoveCameraLeft,
  ZoomInCamera,
  ZoomOutCamera,
  CenterCamera,
  AutoAttack,
  MouseClick(MouseRay)
}

// impl FromIterator<UserInput> for UserInput{
//   fn from_iter<T: IntoIterator<Item = UserInput>>(iter: T) -> Self {
//     iter.into_iter().find(||)
//     match iter {
//       UserInput::MouseClick(ray) => {
//         return UserInput::MouseClick(ray)
//       },
//       _=> {}
//     }
//   }
// }

#[derive(Debug)]
pub struct FrameInputs {
  inputs:Vec<UserInput>
}

impl FrameInputs {
  pub fn new() -> Self {
    FrameInputs { inputs:vec![] }
  }
  // pub fn get_inputs(&self) -> &Vec<UserInput> {
  //   let inputs = &self.inputs;
  //   inputs
  // }

  pub fn get_input(&self)  -> Option<UserInput> {
    let filtered_input = self.inputs.clone().into_iter().find(|input|{
      match input{
        UserInput::MouseClick(_) => true,
        _=> false
      }
    });
    filtered_input
  }

  pub fn add_event(&mut self, event:UserInput) {
    self.inputs.push(event)
  }

  ///Run at the end of each tick to reset the input list.
  pub fn clear(&mut self) {
    //This needs to do something like not clear the ones still being held down?
    self.inputs = vec![]
  }
}
