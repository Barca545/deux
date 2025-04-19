#[derive(Debug,)]
/// Resource containing the state of all movement inputs in the game.
pub struct PlayerMovement {
  // Movement
  pub up: bool,
  pub down: bool,
  pub left: bool,
  pub right: bool,
  // TODO: Analog would hold a vec3 not a bool
}

impl PlayerMovement {
  pub fn new() -> Self {
    PlayerMovement {
      up: false,
      down: false,
      left: false,
      right: false,
    }
  }
}
