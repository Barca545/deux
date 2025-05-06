use math::{convert_screen_coords_to_ndc, raycasting::Ray3D, Mat4, Vec3, Vec4};

#[derive(Debug, Default, Clone, Copy,)]
pub struct MouseRay(pub Ray3D,);

// Where do the screen dimensions come from?
// Camera?
// Inverse?

impl MouseRay {
  // If instead of the camera I just make this a mat4 that'd work to decrease
  // weird dependencies
  pub fn new(x: f64, y: f64, proj_mat: &Mat4, view_mat: &Mat4,) -> Self {
    // At least some of the transforms exist on the camera now

    let inverse_projection: Mat4 = proj_mat.try_inverse().unwrap();
    let inverse_view: Mat4 = view_mat.try_inverse().unwrap();

    // TODO: Why are these not being used? Shouldn't the origin NDC use it?
    let (ndc_x, ndc_y,) = convert_screen_coords_to_ndc(x as f32, y as f32, todo!(), todo!(),);

    // Get the ray's origin in worldspace
    let origin_ndc: Vec4 = Vec4::new(x as f32, y as f32, -1.0, 1.0,);

    // Convert to viewspace
    let mut ray_origin_viewspace_coordinates: Vec4 = inverse_projection * origin_ndc;
    ray_origin_viewspace_coordinates /= ray_origin_viewspace_coordinates.w;

    // Convert to worldspace
    let mut ray_origin_worldspace_coordinates: Vec4 =
      inverse_view * ray_origin_viewspace_coordinates;
    ray_origin_worldspace_coordinates /= ray_origin_worldspace_coordinates.w;

    let end_ndc: Vec4 = Vec4::new(x as f32, y as f32, 0.0, 1.0,);

    //convert to viewspace
    let mut ray_end_viewspace_coordinates: Vec4 = inverse_projection * end_ndc;
    ray_end_viewspace_coordinates /= ray_end_viewspace_coordinates.w;

    //convert to worldspace
    let mut ray_end_worldspace_coordinates: Vec4 = inverse_view * ray_end_viewspace_coordinates;
    ray_end_worldspace_coordinates /= ray_end_worldspace_coordinates.w;

    MouseRay(Ray3D::new(
      ray_origin_worldspace_coordinates.xyz(),
      ray_end_worldspace_coordinates.xyz(),
    ),)
  }

  pub fn ray_ground_intersection(&self,) -> Vec3 {
    self.0.ray_ground_intersection()
  }
}
