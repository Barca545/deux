use glfw::{Key, MouseButton};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InputErrors {
  #[error("GLFW key name not found for {key:?}")]
  KeyNameNotFound { key: Key },
  #[error("No Keybind registered for {key:?}")]
  KeyNotRegistered { key: Key },
  #[error("No Keybind registered for {button:?}")]
  ButtonNotRegistered { button: MouseButton },
}
