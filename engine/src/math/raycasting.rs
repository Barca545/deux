use glm::{self, inverse, vec3, vec4};

use crate::view::camera::Camera;

use super::math::{Mat4, Vec3, Vec4};
use super::Transforms;

#[derive(Debug, Default, Clone, Copy)]
pub struct RayCast {
  pub(crate) origin: Vec3,
  pub(crate) direction: Vec3,
  pub(crate) inverse_direction: Vec3,
}

impl RayCast {
  pub fn new(origin: Vec3, end: Vec3) -> RayCast {
    let direction: Vec3 = (end - origin).normalize();
    //there must be a better way to calculate this inverse
    let inverse_direction: Vec3 = vec3(1.0 / direction.x, 1.0 / direction.y, 1.0 / direction.z);
    RayCast {
      origin,
      direction,
      inverse_direction,
    }
  }

  ///Calculates the point of intersection between a ray cast in world
  /// coordinates and a given plane.
  ///
  /// Concept based on the equation outlined here: https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-plane-and-ray-disk-intersection.html.
  ///
  ///Code pulled from here: https://rosettacode.org/wiki/Find_the_intersection_of_a_line_with_a_plane#Rust.
  ///
  ///The negative signs are different between the two tutorials
  pub fn ray_plane_intersection(&self, plane_normal: Vec3, plane_origin: Vec3) -> Vec3 {
    //checks for the distance where the ray has a point on the plane
    let numerator = (plane_origin - self.origin).dot(&plane_normal);
    let denominator = self.direction.dot(&plane_normal);
    let distance = numerator / denominator;

    //scale is the same as multiplying by distance so benchmark which is faster
    let intersection_point: Vec3 = self.origin + self.direction.scale(distance);
    intersection_point
  }

  ///Calculates the point of intersection between a ray cast in world
  /// coordinates and the ground.
  ///
  /// Concept based on the equation outlined here: https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-plane-and-ray-disk-intersection.html.
  ///
  ///Code pulled from here: https://rosettacode.org/wiki/Find_the_intersection_of_a_line_with_a_plane#Rust.
  ///
  ///The negative signs are different between the two tutorials
  pub fn ray_ground_intersection(&self) -> Vec3 {
    //I think the normal to plane xz is this
    let plane_normal: Vec3 = vec3(0.0, 1.0, 0.0);
    //this is "plane_point" in the tutorial. I think it can just be any point on
    // the plane?
    let plane_origin: Vec3 = vec3(0.0, 0.0, 0.0);

    //checks for the distance where the ray has a point on the plane
    let numerator = (plane_origin - self.origin).dot(&plane_normal);
    let denominator = self.direction.dot(&plane_normal);
    let distance = numerator / denominator;

    //scale is the same as multiplying by distance so benchmark which is faster
    let mut intersection_point: Vec3 = self.origin + self.direction.scale(distance);
    intersection_point.y = 0.0;
    intersection_point
  }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct MouseRay(pub RayCast);

impl MouseRay {
  pub fn new(x: f64, y: f64, transforms: &Transforms, camera: &Camera) -> Self {
    let inverse_projection: Mat4 = transforms.proj_mat().try_inverse().unwrap();
    let inverse_view: Mat4 = inverse(&camera.view_mat());

    // let ndc_x = 2.0 * x as f32 / screen_dimensions.width as f32 - 1.0; //range [-1,1]
    // let ndc_y = 1.0 - (2.0 * y as f32) / screen_dimensions.height as f32; //range [-1,1]

    //get the ray's origin in worldspace
    let origin_ndc: Vec4 = vec4(x as f32, y as f32, -1.0, 1.0);

    //convert to viewspace
    let mut ray_origin_viewspace_coordinates: Vec4 = inverse_projection * origin_ndc;
    ray_origin_viewspace_coordinates /= ray_origin_viewspace_coordinates.w;

    //convert to worldspace
    let mut ray_origin_worldspace_coordinates: Vec4 = inverse_view * ray_origin_viewspace_coordinates;
    ray_origin_worldspace_coordinates /= ray_origin_worldspace_coordinates.w;

    let end_ndc: Vec4 = vec4(x as f32, y as f32, 0.0, 1.0);

    //convert to viewspace
    let mut ray_end_viewspace_coordinates: Vec4 = inverse_projection * end_ndc;
    ray_end_viewspace_coordinates /= ray_end_viewspace_coordinates.w;

    //convert to worldspace
    let mut ray_end_worldspace_coordinates: Vec4 = inverse_view * ray_end_viewspace_coordinates;
    ray_end_worldspace_coordinates /= ray_end_worldspace_coordinates.w;

    MouseRay(RayCast::new(ray_origin_worldspace_coordinates.xyz(), ray_end_worldspace_coordinates.xyz()))
  }

  pub fn ray_ground_intersection(&self) -> Vec3 {
    self.0.ray_ground_intersection()
  }
}
