use sdl2::{keyboard::Keycode, mouse::MouseButton};
use thiserror::Error;

#[derive(Debug, Error,)]
pub enum InputErrors {
  #[error("No Keybind registered for {:?}",key.name())]
  KeyNotRegistered { key:Keycode, },
  #[error("No Keybind registered for {button:?}")]
  ButtonNotRegistered { button:MouseButton, },
}
