use super::math::{Mat4, Perspective, Vec3};
use glm::{identity, scale, translate};

//  Refactor:
// -Model transform should be a component?
// -Screen dimension and Transforms can be merged? I think they're pretty much always accessed together but I should confirm.
// -Restructure the transforms and the camera struct to match the GITGD ECS repositoy from his C++ OpenGL series.
// -Move the glm functions into the math mod

const Z_NEAR: f32 = 0.1;
const Z_FAR: f32 = 100.0;
const DEFAULT_FOV: f32 = 45.0;

#[rustfmt::skip]
const OPENGL_TO_WGPU_MATRIX:Mat4 = Mat4::new(   
  1.0, 0.0, 0.0, 0.0,
  0.0, 1.0, 0.0, 0.0,
  0.0, 0.0, 0.5, 0.5,
  0.0, 0.0, 0.0, 1.0,
);

pub struct Transforms {
  aspect: f32,
  fov: f32,
  znear: f32,
  zfar: f32,
  // camera:Camera
}

impl Transforms {
  pub fn new(aspect: f32) -> Self {
    Transforms {
      aspect,
      fov: DEFAULT_FOV,
      znear: Z_NEAR,
      zfar: Z_FAR,
    }
  }

  pub fn proj_mat(&self) -> Mat4 {
    OPENGL_TO_WGPU_MATRIX * Perspective::new(self.aspect, self.fov, self.znear, self.zfar).as_matrix()
  }
}

pub fn calculate_model_transform(position: &Vec3, scale_factor: f32) -> Mat4 {
  let model: Mat4 = identity::<f32, 4>();
  let model: Mat4 = translate(&model, position);
  let model: Mat4 = scale(&model, &Vec3::new(scale_factor, scale_factor, scale_factor));
  model
}
