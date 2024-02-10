#[derive(Debug, Clone, Copy)]
///Component containing an entity's level.
pub struct Level(pub u32);

impl Default for Level{
  fn default() -> Self {
    Self(1)
  }
}

#[derive(Debug, Clone, Copy, Default)]
///Component containing an entity's experience.
pub struct Exp(pub u32);