use crate::{
  errors::InputErrors,
  math::{MouseRay, Transforms},
  view::{camera::Camera, sdl2_helpers::PhysicalPosition},
};
use eyre::Result;
use nina::world::World;
use sdl2::{keyboard::Keycode, mouse::MouseButton};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Debug};

//Refactor:
// - Keybinds need to be renamed since I want to have the Input struct hold the
//   key action
// - Figure out how to serialize
// - Figure out better way to get the mouse button names
// - Figure out if there is a reason for this to be separate from GameEvents? I
//   think the inputs can possibly just be maped to those here, directly
// - One reason might be so the client and server can be decoupled
// - Need to find ways to store mouse bindings

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,)]
pub enum Keybind {
  AbilityOne,
  AbilityTwo,
  AbilityThree,
  AbilityFour,
  MouseClick,
  // Movement
  Foward,
  Backwards,
  Left,
  Right,
}

#[derive(Debug, Clone,)]
pub struct Keybinds {
  buttons:HashMap<Keycode, Keybind,>,
  mouse:HashMap<MouseButton, Keybind,>,
}

impl Keybinds {
  /// Create an [`Input`] from a [`Keycode`].
  pub fn key_input(
    &self,
    world:&World,
    mouse_pos:&PhysicalPosition,
    key:Keycode,
  ) -> Result<Input,> {
    if let Some(keybind,) = self.buttons.get(&key,) {
      let transforms = world.get_resource::<Transforms>();
      let camera = world.get_resource::<Camera>();
      let mouse = MouseRay::new(mouse_pos.x, mouse_pos.y, &transforms, &camera,);

      Ok(Input {
        mouse,
        keybind:*keybind,
      },)
    }
    else {
      return Err(InputErrors::KeyNotRegistered { key, }.into(),);
    }
  }

  /// Create an [`Input`] from a mouse button click.
  pub fn mouse_input(
    &self,
    world:&World,
    mouse_pos:&PhysicalPosition,
    button:&MouseButton,
  ) -> Result<Input,> {
    if let Some(keybind,) = self.mouse.get(&button,) {
      let transforms = world.get_resource::<Transforms>();
      let camera = world.get_resource::<Camera>();
      let mouse = MouseRay::new(mouse_pos.x, mouse_pos.y, &transforms, &camera,);
      Ok(Input {
        mouse,
        keybind:*keybind,
      },)
    }
    else {
      return Err(InputErrors::ButtonNotRegistered { button:*button, }.into(),);
    }
  }

  /// Returns a [`Keycode`]'s corresponding [`Keybind`].
  pub fn get_input(&self, key:Keycode,) -> Result<Keybind,> {
    if let Some(keybind,) = self.buttons.get(&key,) {
      Ok(*keybind,)
    }
    else {
      return Err(InputErrors::KeyNotRegistered { key, }.into(),);
    }
  }
}

impl Default for Keybinds {
  fn default() -> Self {
    let mut keybinds = Keybinds {
      buttons:HashMap::new(),
      mouse:HashMap::new(),
    };
    keybinds
      .buttons
      .insert(Keycode::from_name("W",).unwrap(), Keybind::Foward,);
    keybinds
      .buttons
      .insert(Keycode::from_name("A",).unwrap(), Keybind::Left,);
    keybinds
      .buttons
      .insert(Keycode::from_name("S",).unwrap(), Keybind::Backwards,);
    keybinds
      .buttons
      .insert(Keycode::from_name("D",).unwrap(), Keybind::Right,);

    keybinds
      .mouse
      .insert(MouseButton::Left, Keybind::MouseClick,);
    keybinds
  }
}

#[derive(Debug, Clone, Copy,)]
pub struct Input {
  pub mouse:MouseRay,
  pub keybind:Keybind,
}

impl Input {
  pub fn new(mouse:MouseRay, keybind:Keybind,) -> Self {
    Self { mouse, keybind, }
  }
}

#[derive(Debug,)]
pub struct FrameInputs {
  inputs:Vec<Input,>,
}

impl FrameInputs {
  pub fn new() -> Self {
    FrameInputs { inputs:vec![], }
  }

  /// Iterates over the [`Input`]s stored in the [`FrameInputs`] and applies a
  /// callback function.
  pub fn process_inputs<F,>(&self, mut f:F,)
  where F: FnMut(&Input,) {
    for input in &self.inputs {
      f(input,)
    }
  }

  /// Add a [`Input`] to the [`FrameInputs`].
  pub fn push(&mut self, input:Input,) {
    self.inputs.push(input,)
  }

  pub fn is_empty(&self,) -> bool {
    self.inputs.is_empty()
  }

  /// Run at the end of each tick to reset the input list.
  pub fn clear(&mut self,) {
    // TODO: This needs to do something like not clear the ones still being held
    // down?
    self.inputs.clear()
  }
}
