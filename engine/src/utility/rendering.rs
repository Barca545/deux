use crate::{
  component_lib::{Position, PreviousPosition},
  math::Vec3,
};
use glm::lerp;

// Refactor:
// -Add lerp to the math crate

pub fn calculate_render_position(previous_position: PreviousPosition, position: Position, interpolation_factor: f64) -> Position {
  let render_position: Vec3;
  if previous_position.0.x == position.0.x && previous_position.0.y == position.0.y && previous_position.0.z == position.0.z {
    render_position = lerp(&previous_position.0, &position.0, interpolation_factor as f32);
  } else {
    render_position = position.0;
  }
  Position(render_position)
}
