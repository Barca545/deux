use glm::{identity, look_at, scale, translate, vec3};
use nalgebra::Perspective3;

use super::math::{radians, Mat4, Vec3};
use crate::view::camera::Camera;

//might make the most sense to make the model transform something the entity holds instead of something the global struct holds
//also restructure this and the camera class to match the ecs one in GITGD's tutorial repo

pub struct Transforms {
  projection_transform:Perspective3<f32>,
  view_transform:Mat4,
  fov:f32,
  camera:Camera //lets have transforms hold the camera
}

impl Transforms {
  pub fn new(aspect:&f32) -> Self {
    let fov = radians(45.0);
    let camera = Camera::new();
    let projection_transform = Self::calculate_projection_transform(fov, aspect);
    let view_transform:Mat4 = Self::calculate_view_transform(&camera);

    Transforms {
      projection_transform,
      view_transform,
      fov,
      camera
    }
  }

  //when does this run
  //updating should be handled by a system
  // pub fn update(&mut self, aspect:&f32, camera:&Camera) {
  //   self.projection_transform = Self::calculate_projection_transform(self.fov, aspect);
  //   self.view_transform = Self::calculate_view_transform(camera);
  // }

  pub fn get_projection_transform(&self) -> Perspective3<f32> {
    self.projection_transform
  }

  pub fn get_view_transform(&self) -> Mat4 {
    self.view_transform
  }

  pub fn calculate_projection_transform(fov:f32, aspect:&f32) -> Perspective3<f32> {
    //Do this without cloning?
    let projection = Perspective3::new(aspect.clone(), fov, 0.1, 100.0);
    projection
  }

  pub fn calculate_view_transform(camera:&Camera) -> Mat4 {
    let view = look_at(&camera.position, &camera.target, &camera.up);
    view
  }
}

pub fn calculate_model_transform(position:&Vec3, scale_factor:f32) -> Mat4 {
  let model:Mat4 = identity::<f32, 4>();
  let model:Mat4 = translate(&model, position);
  let model:Mat4 = scale(&model, &vec3(scale_factor, scale_factor, scale_factor));
  model
}