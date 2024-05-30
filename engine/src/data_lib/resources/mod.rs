#[derive(Debug, Clone, Copy)]
pub enum Selected {
  NONE,
  HOVERED(usize),
  CLICKED(usize)
}

pub struct DebugElements {
  pub aabb:bool,
  pub attacks:bool
}

impl DebugElements {
  pub fn new(aabb:bool, attacks:bool) -> Self {
    DebugElements { aabb, attacks }
  }
}
