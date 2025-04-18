use crate::{errors::InputErrors, mouseray::MouseRay};
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

/// Resource containing the state of all inputs in the game.
pub struct PlayerInputs {
  // Movement
  pub up: bool,
  pub down: bool,
  pub left: bool,
  pub right: bool,
  // TODO: Analog would hold a vec3 not a bool
}

impl PlayerInputs {
  pub fn new() -> Self {
    PlayerInputs {
      up: false,
      down: false,
      left: false,
      right: false,
    }
  }
}

#[derive(Debug, Clone, Copy,)]
pub struct Input {
  /// The location of the mouse at the time of the `Input`.
  pub mouse: MouseRay,
  /// The command the `Input` contains.
  pub keybind: Keybind,
  /// The [`KeyAction`] of the key.
  pub action: Option<KeyAction,>,
}

#[derive(Debug, Clone, Copy,)]
/// Indicates what action occured to a key in an [`Input`].
pub enum KeyAction {
  Press,
  Release,
}

impl Input {
  pub fn new(mouse: MouseRay, keybind: Keybind, action: Option<KeyAction,>,) -> Self {
    Self {
      mouse,
      keybind,
      action,
    }
  }
}

#[derive(Debug,)]
pub struct FrameInputs {
  inputs: Vec<Input,>,
}

impl FrameInputs {
  pub fn new() -> Self {
    FrameInputs { inputs: vec![], }
  }

  /// Iterates over the [`Input`]s stored in the [`FrameInputs`] and applies a
  /// callback function.
  pub fn process_inputs<F,>(&self, mut f: F,)
  where
    F: FnMut(&Input,),
  {
    for input in &self.inputs {
      f(input,)
    }
  }

  /// Add a [`Input`] to the [`FrameInputs`].
  pub fn push(&mut self, input: Input,) {
    self.inputs.push(input,)
  }

  pub fn is_empty(&self,) -> bool {
    self.inputs.is_empty()
  }

  /// Run at the end of each tick to reset the input list.
  pub fn clear(&mut self,) {
    // TODO: This needs to do something like not clear the ones still being held
    // down? Could maybe have an array of stashed inputs or something to send each
    // frame. But I think it's easier to just have stuff rely on assuming something
    // is pressed until it gets a released notif
    self.inputs.clear()
  }
}
