use math::raycasting::RayCast;
use renderer::scene::camera::Camera;

#[derive(Debug, Default, Clone, Copy,)]
pub struct MouseRay(pub RayCast,);

impl MouseRay {
  pub fn new(x:f64, y:f64, camera:&Camera,) -> Self {
    let inverse_projection:Mat4 = transforms.proj_mat().try_inverse().unwrap();
    let inverse_view:Mat4 = inverse(&camera.view_mat(),);

    let ndc_x = 2.0 * x as f32 / screen_dimensions.width as f32 - 1.0; // range   [-1,1]
    let ndc_y = 1.0 - (2.0 * y as f32) / screen_dimensions.height as f32; // range [-1,1]

    // Get the ray's origin in worldspace
    let origin_ndc:Vec4 = vec4(x as f32, y as f32, -1.0, 1.0,);

    // Convert to viewspace
    let mut ray_origin_viewspace_coordinates:Vec4 = inverse_projection * origin_ndc;
    ray_origin_viewspace_coordinates /= ray_origin_viewspace_coordinates.w;

    // Convert to worldspace
    let mut ray_origin_worldspace_coordinates:Vec4 =
      inverse_view * ray_origin_viewspace_coordinates;
    ray_origin_worldspace_coordinates /= ray_origin_worldspace_coordinates.w;

    let end_ndc:Vec4 = vec4(x as f32, y as f32, 0.0, 1.0,);

    //convert to viewspace
    let mut ray_end_viewspace_coordinates:Vec4 = inverse_projection * end_ndc;
    ray_end_viewspace_coordinates /= ray_end_viewspace_coordinates.w;

    //convert to worldspace
    let mut ray_end_worldspace_coordinates:Vec4 = inverse_view * ray_end_viewspace_coordinates;
    ray_end_worldspace_coordinates /= ray_end_worldspace_coordinates.w;

    MouseRay(RayCast::new(
      ray_origin_worldspace_coordinates.xyz(),
      ray_end_worldspace_coordinates.xyz(),
    ),)
  }

  pub fn ray_ground_intersection(&self,) -> Vec3 {
    self.0.ray_ground_intersection()
  }
}
