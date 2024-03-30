use glm::{inverse, vec4};

use crate::math::{
  math::{Mat4, Vec4},
  Dimensions, MouseRay, Transforms,
};

// Refactor:
// -Delete this? I don't think it is ever used

pub struct MousePicker {
  ray: Option<MouseRay>,
}

impl MousePicker {
  pub fn new() -> Self {
    MousePicker { ray: None }
  }

  ///Updates the `MousePicker`'s stored `MouseRay`.
  pub fn update_ray(&mut self, x: f64, y: f64, screen_dimensions: &Dimensions, transforms: &Transforms) {
    let inverse_projection: Mat4 = transforms.projection_transform.inverse();
    let inverse_view: Mat4 = inverse(&transforms.view_transform);

    let ndc_x = 2.0 * x as f32 / screen_dimensions.width as f32 - 1.0; //range [-1,1]
    let ndc_y = 1.0 - (2.0 * y as f32) / screen_dimensions.height as f32; //range [-1,1]

    let ndc: Vec4 = vec4(ndc_x, ndc_y, -1.0, 1.0);

    let mut ray_viewspace_coordinates: Vec4 = inverse_projection * ndc;
    ray_viewspace_coordinates /= ray_viewspace_coordinates.w;

    //convert to worldspace
    let mut ray_worldspace_coordinates: Vec4 = inverse_view * ray_viewspace_coordinates;
    ray_worldspace_coordinates /= ray_worldspace_coordinates.w;

    self.ray = Some(MouseRay::new(x, y, screen_dimensions, transforms));
  }
}
