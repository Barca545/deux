use serde::{Deserialize, Serialize};
use crate::physics::AABB3D;
use super::movement::Position;

#[derive(Debug, Clone, Copy)]
///Component containing an entity's 3D AABB for unit selection.
pub struct SelectionRadius(pub AABB3D);
impl SelectionRadius {
  pub fn new(position:&Position, height:f32, radius:f32) -> Self {
    let aabb3d = AABB3D::new(position.0, height, radius);
    SelectionRadius(aabb3d)
  }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
///Component containing an entity's radius for edge-to-edge gameplay logic.
pub struct GameplayRadius(pub f32);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
///Component containing an entity's radius for unit collision and pathing logic.
pub struct PathingRadius(pub f32);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
///Component containing the radius of vision an entity provides around itself.
pub struct VisionRadius(f32);