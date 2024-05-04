use thiserror::Error;
use winit::{event::MouseButton, keyboard::PhysicalKey as Key};

#[derive(Debug, Error)]
pub enum InputErrors {
  #[error("No Keybind registered for {key:?}")]
  KeyNotRegistered { key: Key },
  #[error("No Keybind registered for {button:?}")]
  ButtonNotRegistered { button: MouseButton },
}
