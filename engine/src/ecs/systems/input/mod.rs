// Refactor:
// -Need system to convert button/mouse inputs into game events
// -Need system that handles selections (clicking).
// -Need separate system that handles UI selection stuff i.e. hovering over an element
mod update_target;

pub use self::update_target::{process_inputs, update_target};

pub mod update_mouseray;
