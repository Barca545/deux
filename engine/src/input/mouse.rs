use crate::view::camera::Camera;
use glm::{Vec3, Mat4};

#[derive(Debug)]
pub struct Mouse{
  current_ray:Vec3,
  projection_matrix:Mat4,
  camera:Camera
}