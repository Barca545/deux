use std::fmt::Debug;

//this is lazy, probably shouldn't be public but whatever
#[derive(Debug)]
pub struct MousePosition {
  pub x:f64,
  pub y:f64
}

#[derive(Debug)]
pub enum UserInputs {
  MoveCameraUp,
  MoveCameraDown,
  MoveCameraRight,
  MoveCameraLeft,
  ZoomInCamera,
  ZoomOutCamera,
  CenterCamera,
  AutoAttack,
  MouseClick(MousePosition) //place holder so the matches don't say the "_>" term is unreachable
}

#[derive(Debug)]
pub struct FrameInputs {
  inputs:Vec<UserInputs>
}

impl FrameInputs {
  pub fn new() -> Self {
    FrameInputs { inputs:vec![] }
  }
  pub fn get_inputs(&self) -> &Vec<UserInputs> {
    let inputs = &self.inputs;
    inputs
  }

  pub fn add_event(&mut self, event:UserInputs) {
    self.inputs.push(event)
  }

  ///Run at the end of each tick to reset the input list.
  pub fn clear(&mut self) {
    //This needs to do something like not clear the ones still being held down?
    self.inputs = vec![]
  }
}
