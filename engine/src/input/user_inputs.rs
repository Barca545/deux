use glfw::{get_key_name, Key, MouseButton, Window};
use serde::{Deserialize, Serialize};

use crate::{
  ecs::{world_resources::ScreenDimensions, World},
  errors::InputErrors,
  math::{MouseRay, Transforms},
};
use eyre::Result;
use std::{collections::HashMap, fmt::Debug};

//Refactor:
// -Keybinds need to be renamed since I want to have the Input struct hold the key action
// -Figure out how to serialize
// -Figure out better way to get the mouse button names
// -Figure out if there is a reason for this to be separate from GameEvents? I think the inputs can possibly just be maped to those here, directly
//  One reason might be so the client and server can be decoupled

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Keybind {
  AbilityOne = 0,
  AbilityTwo = 1,
  AbilityThree = 2,
  AbilityFour = 3,
  MouseClick = 4,
}

#[derive(Debug, Clone)]
pub struct Keybinds(HashMap<String, Keybind>);
impl Keybinds {
  ///Create an [`Input`] from a [`Key`].
  pub fn key_input(&self, world: &World, window: &Window, key: Key) -> Result<Input> {
    if let Some(keystring) = key.get_name() {
      if let Some(keybind) = self.0.get(&keystring) {
        let (x, y) = window.get_cursor_pos();
        let screen_dimensions = world.get_resource::<ScreenDimensions>().unwrap();
        let transforms = world.get_resource::<Transforms>().unwrap();
        let mouse = MouseRay::new(x, y, &screen_dimensions, &transforms);
        Ok(Input { mouse, keybind: *keybind })
      } else {
        return Err(InputErrors::KeyNotRegistered { key }.into());
      }
    } else {
      return Err(InputErrors::KeyNameNotFound { key }.into());
    }
  }

  ///Create an [`Input`] from a [`Key`].
  pub fn mouse_input(&self, world: &World, window: &Window, button: MouseButton) -> Result<Input> {
    let buttonstring = format!("{:?}", button);
    if let Some(keybind) = self.0.get(&buttonstring) {
      let (x, y) = window.get_cursor_pos();
      let screen_dimensions = world.get_resource::<ScreenDimensions>().unwrap();
      let transforms = world.get_resource::<Transforms>().unwrap();
      let mouse = MouseRay::new(x, y, &screen_dimensions, &transforms);
      Ok(Input { mouse, keybind: *keybind })
    } else {
      return Err(InputErrors::ButtonNotRegistered { button }.into());
    }
  }

  ///Returns a [`Key`]'s corresponding [`Keybind`].
  pub fn get_input(&self, key: Key, scancode: i32) -> Result<Keybind> {
    if let Some(keystring) = get_key_name(Some(key), Some(scancode)) {
      if let Some(keybind) = self.0.get(&keystring) {
        Ok(*keybind)
      } else {
        return Err(InputErrors::KeyNotRegistered { key }.into());
      }
    } else {
      return Err(InputErrors::KeyNameNotFound { key }.into());
    }
  }
}

impl Default for Keybinds {
  fn default() -> Self {
    let mut keybinds = Keybinds(HashMap::new());
    keybinds.0.insert(String::from("Button2"), Keybind::MouseClick);
    keybinds.0.insert(String::from("q"), Keybind::AbilityOne);
    keybinds.0.insert(String::from("w"), Keybind::AbilityTwo);
    keybinds.0.insert(String::from("e"), Keybind::AbilityThree);
    keybinds.0.insert(String::from("r"), Keybind::AbilityFour);
    keybinds
  }
}

#[derive(Debug, Clone, Copy)]
pub struct Input {
  pub mouse: MouseRay,
  pub keybind: Keybind,
}

impl Input {
  pub fn new(mouse: MouseRay, keybind: Keybind) -> Self {
    Self { mouse, keybind }
  }
}

#[derive(Debug)]
pub struct FrameInputs {
  inputs: Vec<Input>,
}

impl FrameInputs {
  pub fn new() -> Self {
    FrameInputs { inputs: vec![] }
  }

  ///Iterates over the [`Input`]s stored in the [`FrameInputs`] and applies a callback function.
  pub fn process_inputs<F>(&self, mut f: F)
  where
    F: FnMut(&Input),
  {
    for input in &self.inputs {
      f(input)
    }
  }

  ///Add a [`Input`] to the [`FrameInputs`].
  pub fn push(&mut self, input: Input) {
    self.inputs.push(input)
  }

  ///Run at the end of each tick to reset the input list.
  pub fn clear(&mut self) {
    //This needs to do something like not clear the ones still being held down?
    self.inputs.clear()
  }
}

#[cfg(test)]
mod test {
  use crate::{
    ecs::{world_resources::ScreenDimensions, World},
    input::user_inputs::Keybind,
    math::Transforms,
  };
  use eyre::Result;
  use glfw::Key;

  use super::Keybinds;

  #[test]
  fn get_user_input_from_keybinds() -> Result<()> {
    let keybinds = Keybinds::default();
    let mut world = World::new();
    let screen_dimensions = ScreenDimensions::new(1280, 720);
    let transforms = Transforms::new(&screen_dimensions.aspect);
    world.add_resource(screen_dimensions).add_resource(transforms);
    let q = keybinds.get_input(Key::Q, 16)?;

    assert_eq!(q, Keybind::AbilityOne);
    Ok(())
  }
}
