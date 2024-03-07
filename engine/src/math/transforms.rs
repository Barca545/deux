use glm::{identity, look_at, scale, translate};
use nalgebra::Perspective3;

use super::math::{radians, Mat4, Vec3};
use crate::view::camera::Camera;

//  Refactor:
// -Model transform should be a component?
// -Screen dimension and Transforms can be merged? I think they're pretty much always accessed together but I should confirm.
// -Restructure the transforms and the camera struct to match the GITGD ECS repositoy from his C++ OpenGL series.
// -Move the glm functions into the math mod

pub struct Transforms {
  pub projection_transform: Perspective3<f32>,
  pub view_transform: Mat4,
  //fov and camera will be used when I make a camera system
  // fov:f32,
  // camera:Camera
}

impl Transforms {
  pub fn new(aspect: &f32) -> Self {
    let fov = radians(45.0);
    let camera = Camera::new();
    let view_transform: Mat4 = Self::calculate_view_transform(&camera);
    let projection_transform = Self::calculate_projection_transform(fov, aspect);

    Transforms {
      projection_transform,
      view_transform,
      // fov,
      // camera
    }
  }

  fn calculate_view_transform(camera: &Camera) -> Mat4 {
    let view = look_at(&camera.position, &camera.target, &camera.up);
    view
  }

  fn calculate_projection_transform(fov: f32, aspect: &f32) -> Perspective3<f32> {
    let projection = Perspective3::new(*aspect, fov, 0.1, 100.0);
    projection
  }
}

pub fn calculate_model_transform(position: &Vec3, scale_factor: f32) -> Mat4 {
  let model: Mat4 = identity::<f32, 4>();
  let model: Mat4 = translate(&model, position);
  let model: Mat4 = scale(&model, &Vec3::new(scale_factor, scale_factor, scale_factor));
  model
}
